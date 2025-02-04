mod functions;

use std::fs;
use axum::{Router, routing::{get,post}, response::{Html, IntoResponse}, Json};
use serde_json::json;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

async fn create_user_handler(Json((email,password,_username)): Json<(String,String,String)>) -> impl IntoResponse {
    let response = functions::create_user(email,password,_username);
    Json(json!(response))
}


pub fn create_router() -> Router {
    let mut router = Router::new()
    .route("/create_user", post(create_user_handler));
router = router.nest_service(
                        "/assets",
                        ServeDir::new(&format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "src/static/assets")),
                    );let router = router.layer(LiveReloadLayer::new());

       let router = router.fallback(get({
        match fs::read_to_string(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "src/static/index.html"
        )) {
            Ok(contents) => Html(contents),
            Err(_) => Html("Error reading file".to_string()),
        }
    }));
return router;
}
