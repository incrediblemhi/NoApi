pub mod functions;
pub mod handlers;

use std::fs;

use handlers::create_router;
use listenfd::ListenFd;
use noapi_functions::struct_extractor::extract_struct_def;
use tokio::net::TcpListener;

#[derive(serde::Serialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[tokio::main]
async fn main() {
    let app = create_router();

    let content = fs::read_to_string(&format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "src/functions.rs"
    ))
    .expect("Failed to read Rust file");
    let use_statements = extract_struct_def("crate::User".to_string(), &content);

    println!("{:#?}", use_statements);

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
