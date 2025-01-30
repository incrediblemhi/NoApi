mod functions;

use axum::{Router, routing::post, response::IntoResponse, Json};
use serde_json::json;

async fn add_handler(Json((num1,num2)): Json<(u32,u32)>) -> impl IntoResponse {
    let response = functions::add(num1,num2);
    Json(json!(response))
}


pub fn create_router() -> Router {
    Router::new()
    .route("/add", post(add_handler))
}
