pub mod functions;
pub mod handlers;

use handlers::create_router;
use listenfd::ListenFd;
use noapi_functions::build_functions::cargo_doc_path;
use tokio::net::TcpListener;

#[derive(serde::Serialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[tokio::main]
async fn main() {
    let app = create_router();

    println!("{}", cargo_doc_path().as_path().to_str().unwrap());

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
