pub mod build_functions;

use axum::{response::Html, routing::get, Router};
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::Arc,
};
use tower_http::services::ServeDir;

async fn serve_html(file_path: Arc<String>) -> Html<String> {
    // Read the contents of the file
    match fs::read_to_string(&*file_path) {
        Ok(contents) => Html(contents),
        Err(_) => Html("Error reading file".to_string()),
    }
}

pub fn generate_routes_from_folder(base_dir: &str, mut router: Router) -> Router {
    // Iterate over the directories in the base folder
    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Check if it's a directory and contains an index.html
            if path.is_dir() {
                let index_file = path.join("index.html");
                if index_file.exists() {
                    // Create a route based on the directory name
                    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                        let route_path = format!("/{dir_name}");
                        let file_path = Arc::new(index_file.to_string_lossy().to_string());

                        // Add the route to the router
                        router = router.route(
                            &route_path,
                            get({
                                let file_path: Arc<String> = file_path.clone();
                                move || serve_html(file_path.clone())
                            }),
                        );
                    }
                } else if path.as_path() == Path::new(&format!("{}{}", base_dir, "/assets")) {
                    router = router.nest_service(
                        "/assets",
                        ServeDir::new(&format!("{}{}", base_dir, "/assets")),
                    )
                }
            } else if path.is_file() {
                if path.as_path().file_name().unwrap().to_str().unwrap() == "index.html" {
                    let file_path = Arc::new(path.to_string_lossy().to_string());

                    router = router.route(
                        &"/",
                        get({
                            let file_path = file_path.clone();
                            move || serve_html(file_path.clone())
                        }),
                    );
                }
            }
        }
    }

    router
}

fn rust_to_typescript_type(rust_type: &str, custom_types: &Vec<String>) -> String {
    match rust_type {
        "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),
        "String" | "&str" => "string".to_string(),
        "bool" => "boolean".to_string(),
        "Vec" => "Array<any>".to_string(),
        "Option" => "any | null".to_string(),
        _ if custom_types.contains(&rust_type.to_string()) => rust_type.to_string(),
        _ => "any".to_string(), // Fallback for unknown types
    }
}

pub fn rust_to_typescript_functons(file_path: &str, output_path: &str) {
    let content = fs::read_to_string(file_path).expect("Failed to read Rust file");

    // Regular expressions to extract structs, enums, and function signatures
    let struct_regex = Regex::new(r"struct\s+(\w+)\s*\{([^}]*)\}").expect("Invalid struct regex");
    let function_regex =
        Regex::new(r"fn\s+(\w+)\s*\(([^)]*)\)\s*->\s*([^{]+)").expect("Invalid function regex");

    let mut ts_content = String::from("import axios from 'axios'\n\n");
    let mut custom_types = Vec::new();

    // Extract and generate TypeScript interfaces for structs
    for cap in struct_regex.captures_iter(&content) {
        let struct_name = &cap[1];
        let fields = &cap[2];
        custom_types.push(struct_name.to_string());

        let ts_fields: Vec<String> = fields
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.trim().split(':').collect();
                if parts.len() == 2 {
                    let field_name = parts[0].trim();
                    let field_type = rust_to_typescript_type(
                        parts[1].trim().trim_end_matches(','),
                        &custom_types,
                    );
                    format!("    {}: {};", field_name, field_type)
                } else {
                    String::new()
                }
            })
            .collect();

        ts_content.push_str(&format!(
            "export interface {} {{\n{}\n}}\n\n",
            struct_name,
            ts_fields.join("\n")
        ));
    }

    // Extract and generate TypeScript function signatures
    for cap in function_regex.captures_iter(&content) {
        let function_name = &cap[1];
        let params = &cap[2];
        let return_type = &cap[3].trim();

        // Parse parameters

        let mut ts_params_without_types: Vec<String> = vec![];
        let ts_params: Vec<String> = params
            .split(',')
            .filter(|p| !p.trim().is_empty())
            .map(|p| {
                let parts: Vec<&str> = p.trim().split(':').collect();
                if parts.len() == 2 {
                    let param_name = parts[0].trim();
                    match parts[1].trim() {
                        "i32" | "i64" | "u32" | "u64" => ts_params_without_types.push(format!(
                            "parseInt({}.toString(), 10)",
                            param_name.to_string()
                        )),
                        "f32" | "f64" => ts_params_without_types
                            .push(format!("parseFloat({}.toString())", param_name.to_string())),
                        _ => (),
                    }
                    let param_type = rust_to_typescript_type(parts[1].trim(), &custom_types);
                    format!("{}: {}", param_name, param_type)
                } else {
                    "unknown: any".to_string()
                }
            })
            .collect();

        // Convert return type
        let ts_return_type = rust_to_typescript_type(return_type, &custom_types);

        // Add to TypeScript content
        ts_content.push_str(&format!(
            "export async function {}({}): Promise<{}>{{\nlet data:any = [{}];\n let response = await axios.post('{}', data);\n return response.data;\n}}\n\n",
            function_name,
            ts_params.join(", "),
            ts_return_type,
            ts_params_without_types.join(", "),
            format!("http://localhost:3000/{}",function_name),
        ));
    }

    // Write to TypeScript file
    let mut ts_file = File::create(output_path).expect("Failed to create TypeScript file");
    ts_file
        .write_all(ts_content.as_bytes())
        .expect("Failed to write to TypeScript file");

    println!("TypeScript file generated at {}", output_path);
}
