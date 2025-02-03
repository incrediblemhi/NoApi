use noapi_functions::build_functions::{
    build_frontend, rust_functions_to_axum_handlers, rust_to_typescript_functons,
};

fn main() {
    println!("cargo:rerun-if-changed=./src/functions.rs");
    rust_functions_to_axum_handlers("./src/functions.rs", "./src/handlers");
    println!("cargo:rerun-if-changed=./src/functions.rs");
    rust_to_typescript_functons("./src/functions.rs", "./functions.ts");
    println!("cargo:rerun-if-changed=frontend");
    build_frontend().unwrap()
}
