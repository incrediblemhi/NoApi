use regex::Regex;
use std::{fs, io::Write, process::Command};

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
        "mod functions;\n\nuse axum::{Router, routing::post, response::IntoResponse, Json};\nuse serde_json::json;\n\n",
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
    axum_content.push_str("\npub fn create_router() -> Router {\n    Router::new()\n");
    axum_content.push_str(&routes.join("\n"));
    axum_content.push_str("\n}\n");

    // Write to output file
    let mut file = fs::File::create(format!("{}/mod.rs", output_path))
        .expect("Failed to create output Rust file");
    file.write_all(axum_content.as_bytes())
        .expect("Failed to write to output Rust file");
}
