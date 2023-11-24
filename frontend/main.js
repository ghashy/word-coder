"use sctrict";

const input = document.getElementById("text_input");
const button = document.getElementById("submit_button");

let my_url = "https://ghashy.ru/api/";

button.addEventListener("click", submit_data);

input.addEventListener("keypress", (event) => {
  if (event.key === "Enter") {
    submit_data();
  }
});

function submit_data() {
  let request_url = my_url + input.value;

  fetch(request_url, {
    method: "POST",
  })
    .then((response) => response.json())
    .then((data) => {
      console.log(data);
      let response_div = document.getElementById("response_ul");
      let block = "";
      for (let item of data) {
        block += "<li>";
        block += item;
        block += "</li>";
      }

      console.log(block);
      response_div.innerHTML = block;
    })
    .catch((error) => console.error("Error:", error));
}
