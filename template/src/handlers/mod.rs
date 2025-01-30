mod functions;

use axum::{Router, routing::post, response::IntoResponse, Json};
use serde_json::json;

async fn add_handler(Json((email,password)): Json<(String,String)>) -> impl IntoResponse {
    let response = functions::add(email,password);
    Json(json!(response))
}


pub fn create_router() -> Router {
    Router::new()
    .route("/add", post(add_handler))
}
