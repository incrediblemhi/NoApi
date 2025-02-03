use regex::Regex;
use std::{fs, io::Write, path::PathBuf, process::Command};

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

pub fn build_frontend() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new(NPM)
        .arg("run")
        .arg("build")
        .arg("--emptyOutDir")
        .status()?;

    if !status.success() {
        return Err(format!(
            "Failed to build the frontend, exit code: {:?}",
            status.code()
        )
        .into());
    }

    Ok(())
}

pub fn rust_functions_to_axum_handlers(input_path: &str, output_path: &str) {
    let content = fs::read_to_string(input_path).expect("Failed to read Rust file");

    let mut file = fs::File::create(format!("{}/functions.rs", output_path))
        .expect("Failed to create output Rust file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to output Rust file");

    // Regex to extract function names and parameters
    let function_regex = Regex::new(r"fn\s+(\w+)\s*\(([^)]*)\)").expect("Invalid function regex");

    let mut axum_content = String::from(
        "mod functions;\n\nuse std::fs;\nuse axum::{Router, routing::{get,post}, response::{Html, IntoResponse}, Json};\nuse serde_json::json;\nuse tower_http::services::ServeDir;\nuse tower_livereload::LiveReloadLayer;\n\n",
    );

    let mut routes = Vec::new();

    // Extract functions and generate Axum handlers
    for cap in function_regex.captures_iter(&content) {
        let function_name = &cap[1];
        let params = &cap[2];

        let mut param_names = Vec::new();
        let mut param_types = Vec::new();

        params
            .split(',')
            .filter(|p| !p.trim().is_empty())
            .for_each(|p| {
                let parts: Vec<&str> = p.trim().split(':').collect();
                if parts.len() == 2 {
                    let param_name = parts[0].trim();
                    let param_type = parts[1].trim();
                    param_names.push(param_name);
                    param_types.push(param_type);
                }
            });

        // Create an Axum handler for the function
        let handler_name = format!("{}_handler", function_name);
        let handler = format!(
            "async fn {}(Json(({})): Json<({})>) -> impl IntoResponse {{\n    let response = functions::{}({});\n    Json(json!(response))\n}}\n\n",
            handler_name,param_names.join(","),param_types.join(","), function_name,param_names.join(","),
        );

        axum_content.push_str(&handler);
        routes.push(format!(
            "    .route(\"/{0}\", post({0}_handler))",
            function_name
        ));
    }

    // Create Axum router
    axum_content
        .push_str("\npub fn create_router() -> Router {\n    let mut router = Router::new()\n");
    axum_content.push_str(&routes.join("\n"));
    axum_content.push_str(";\n");
    axum_content.push_str("router = router.nest_service(
                        \"/assets\",
                        ServeDir::new(&format!(\"{}/{}\", env!(\"CARGO_MANIFEST_DIR\"), \"src/static/assets\")),
                    );");
    axum_content.push_str("let router = router.layer(LiveReloadLayer::new());\n");
    axum_content.push_str(
        "
       let router = router.fallback(get({
        match fs::read_to_string(format!(
            \"{}/{}\",
            env!(\"CARGO_MANIFEST_DIR\"),
            \"src/static/index.html\"
        )) {
            Ok(contents) => Html(contents),
            Err(_) => Html(\"Error reading file\".to_string()),
        }
    }));\n",
    );
    axum_content.push_str("return router;");
    axum_content.push_str("\n}\n");

    // Write to output file
    let mut file = fs::File::create(format!("{}/mod.rs", output_path))
        .expect("Failed to create output Rust file");
    file.write_all(axum_content.as_bytes())
        .expect("Failed to write to output Rust file");
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
                        _ => ts_params_without_types.push(param_name.to_string()),
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
    let mut ts_file = fs::File::create(output_path).expect("Failed to create TypeScript file");
    ts_file
        .write_all(ts_content.as_bytes())
        .expect("Failed to write to TypeScript file");

    println!("TypeScript file generated at {}", output_path);
}

pub fn cargo_doc_path() -> PathBuf {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata = serde_json::from_slice::<serde_json::Value>(&output.stdout)
        .expect("Failed to parse cargo metadata");

    let workspace_root = metadata["workspace_root"]
        .as_str()
        .expect("Failed to get workspace root");

    PathBuf::from(workspace_root).join("target/doc")
}

pub fn extract_struct_info(import_path: &str) -> Option<(String, Vec<(String, String)>)> {
    // Extract the struct name
    let struct_name = import_path.split("::").last()?.to_string();

    // Path to the expected HTML file
    let doc_path = cargo_doc_path().join(format!("{}.html", struct_name));

    if !doc_path.exists() {
        println!("Documentation file not found: {:?}", doc_path);
        return None;
    }

    // Read the HTML file
    let html_content = fs::read_to_string(&doc_path).ok()?;

    // Regex to find struct definition (e.g., `struct MyStruct`)
    let struct_regex = Regex::new(r#"struct\s+([A-Za-z0-9_]+)"#).unwrap();

    // Regex to extract fields (e.g., `field_name: Type`)
    let field_regex =
        Regex::new(r#"<span class="structfield">([a-zA-Z0-9_]+)</span>:\s*<code>(.*?)</code>"#)
            .unwrap();

    // Find struct definition
    let struct_def = struct_regex
        .captures(&html_content)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| "Unknown Struct".to_string());

    // Find struct fields and their types
    let fields: Vec<(String, String)> = field_regex
        .captures_iter(&html_content)
        .map(|cap| {
            let field_name = cap.get(1).unwrap().as_str().to_string();
            let field_type = cap.get(2).unwrap().as_str().to_string();
            (field_name, field_type)
        })
        .collect();

    Some((struct_def, fields))
}
