use noapi_functions::{build_frontend, rust_functions_to_axum_handlers};

fn main() {
    rust_functions_to_axum_handlers("./src/functions.rs", "./src/handlers");
    println!("cargo:rerun-if-changed=frontend");
    build_frontend().unwrap()
}
