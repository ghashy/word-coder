use std::io::Result;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::http::Method;
use axum::http::StatusCode;
use axum::routing;
use axum::Json;
use axum::Router;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

mod generator;

#[derive(Clone)]
struct AppState {
    dictionary: Arc<String>,
}

enum Env {
    Dev,
    Prod,
}

impl Env {
    fn init() -> Self {
        match std::env::var("APP_ENVIRONMENT") {
            Ok(s) => match s.as_str() {
                "production" => Env::Prod,
                _ => Env::Dev,
            },
            Err(_) => Env::Dev,
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to setup tracing subscriber");
    let dictionary = generator::read_file_to_string_utf8("russian-POS.txt")?;
    let state = AppState {
        dictionary: Arc::new(dictionary),
    };
    let env = Env::init();
    let app = get_app(state, &env);

    let sock_addr = &std::net::SocketAddr::V4(match env {
        Env::Dev => SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9090),
        Env::Prod => SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9090),
    });

    let server = axum::Server::bind(sock_addr).serve(app.into_make_service());
    let _ = server.await;

    tracing::info!("Stopping..");

    Ok(())
}

fn get_app(state: AppState, env: &Env) -> Router {
    let router = Router::new()
        .route("/api/:number", routing::post(get_words))
        .with_state(state);
    match env {
        Env::Dev => {
            tracing::info!("Running in dev env");
            let cors = CorsLayer::new()
                // allow `GET` and `POST` when accessing the resource
                .allow_methods([Method::GET, Method::POST])
                // allow requests from any origin
                .allow_origin(Any);

            router.layer(cors)
        }
        Env::Prod => {
            tracing::info!("Running in prod env");
            router
        }
    }
}

async fn get_words(
    State(state): State<AppState>,
    Path(number): Path<String>,
) -> (StatusCode, Json<Vec<String>>) {
    if !number.parse::<u32>().is_ok() {
        tracing::info!("Bad request: {}", number);
        return (StatusCode::BAD_REQUEST, Json(Vec::new()));
    }

    match generator::generate_words(&number, &state.dictionary)
        .map(|words| words.iter().map(|&s| s.to_owned()).collect::<Vec<_>>())
    {
        Ok(words) => {
            tracing::info!("Requested: {}", number);
            (StatusCode::OK, Json(words))
        }
        Err(e) => {
            tracing::info!("Failed to parse: {}, error: {}", number, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
        }
    }
}
