use crate::{
    config::{Config, PackageManager},
    js_commands::{bun_build, deno_build, npm_build, pnpm_build, yarn_build},
    struct_extractor::extract_struct_def,
};
use regex::Regex;
use std::{fs, io::Write, path::Path, process::ExitStatus};

pub fn build_frontend() -> Result<(), Box<dyn std::error::Error>> {
    let package_manager = Config::from_file().unwrap().package_manager;
    let status: ExitStatus;

    match package_manager {
        PackageManager::Bun => {
            status = bun_build()?;
        }
        PackageManager::Deno => {
            status = deno_build()?;
        }
        PackageManager::NPM => {
            status = npm_build()?;
        }
        PackageManager::PNPM => {
            status = pnpm_build()?;
        }
        PackageManager::YARN => {
            status = yarn_build()?;
        }
    }

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
                    );\n");
    axum_content.push_str(
        "
       router = router.fallback(get({
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
    axum_content.push_str("router = router.layer(LiveReloadLayer::new());\n");
    axum_content.push_str("return router;");
    axum_content.push_str("\n}\n");

    // Write to output file
    let mut file = fs::File::create(format!("{}/mod.rs", output_path))
        .expect("Failed to create output Rust file");
    file.write_all(axum_content.as_bytes())
        .expect("Failed to write to output Rust file");
}

pub fn rust_to_typescript_type(
    rust_type: &str,
    custom_types: &Vec<String>,
    content: &str,
    project_name: &str,
) -> String {
    match rust_type {
        "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),
        "String" | "&str" => "string".to_string(),
        "bool" => "boolean".to_string(),
        "Vec" => "Array<any>".to_string(),
        "Option" => "any | null".to_string(),
        _ if custom_types.contains(&rust_type.to_string()) => rust_type.to_string(),
        _ => match extract_struct_def(rust_type.to_string(), content, project_name) {
            Some(ts_type) => ts_type,
            None => "any".to_string(),
        },
    }
}

pub fn rust_to_typescript_functons<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    project_name: &str,
) {
    let content = fs::read_to_string(input_path).expect("Failed to read Rust file");
    let struct_regex = Regex::new(r"struct\s+(\w+)\s*\{([^}]*)\}").expect("Invalid struct regex");
    let function_regex =
        Regex::new(r"fn\s+(\w+)\s*\(([^)]*)\)\s*->\s*([^{]+)").expect("Invalid function regex");

    let mut ts_content = String::from("import axios from 'axios'\n\n");
    let mut custom_types = Vec::new();

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
                        &content,
                        project_name,
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
            ts_fields.join("\n").replace("pub", "")
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
                    let param_type = rust_to_typescript_type(
                        parts[1].trim(),
                        &custom_types,
                        &content,
                        project_name,
                    );
                    format!("{}: {}", param_name, param_type)
                } else {
                    "unknown: any".to_string()
                }
            })
            .collect();

        // Convert return type
        let ts_return_type =
            rust_to_typescript_type(return_type, &custom_types, &content, project_name);

        // Add to TypeScript content
        ts_content.push_str(&format!(
            "export async function {}({}): Promise<{}>{{\nlet base_url = window.origin;\nlet data:any = [{}];\n let response = await axios.post(`${{base_url}}/{}`, data);\n return response.data;\n}}\n\n",
            function_name,
            ts_params.join(", "),
            ts_return_type,
            ts_params_without_types.join(", "),
            function_name,
        ));
    }

    // Write to TypeScript file
    let mut ts_file = fs::File::create(output_path).expect("Failed to create TypeScript file");
    ts_file
        .write_all(ts_content.as_bytes())
        .expect("Failed to write to TypeScript file");
}
