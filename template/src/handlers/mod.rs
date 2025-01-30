mod functions;

use axum::{Router, routing::post, response::IntoResponse, Json};
use serde_json::json;

async fn hello_handler(Json((num1,num2)): Json<(u32,u32)>) -> impl IntoResponse {
    let response = functions::hello(num1,num2);
    Json(json!(response))
}


pub fn create_router() -> Router {
    Router::new()
    .route("/hello", post(hello_handler))
}
