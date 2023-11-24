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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let dictionary = generator::read_file_to_string_utf8("russian-POS.txt")?;
    let state = AppState {
        dictionary: Arc::new(dictionary),
    };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/:number", routing::post(get_words))
        .layer(cors)
        .with_state(state);

    let server = axum::Server::bind(&std::net::SocketAddr::V4(
        // SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9090),
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9090),
    ))
    .serve(app.into_make_service());
    let _ = server.await;

    tracing::info!("Stopping..");

    Ok(())
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
