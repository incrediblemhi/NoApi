pub mod functions;
pub mod handlers;

use handlers::create_router;
use listenfd::ListenFd;
use tokio::net::TcpListener;
// imports from cargo spaces
use noapi_functions::{generate_routes_from_folder, rust_to_typescript_functons};

const STATIC_DIR: &str = "./src/static";

#[tokio::main]
async fn main() {
    rust_to_typescript_functons("./src/functions.rs", "./functions.ts");

    let app = create_router();

    let app = generate_routes_from_folder(STATIC_DIR, app);

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
