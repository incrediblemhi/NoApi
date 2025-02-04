use regex::Regex;
use std::{fs, path::PathBuf, process::Command};

use crate::build_functions::rust_to_typescript_type;

fn cargo_doc_path() -> PathBuf {
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

pub fn extract_struct_def(struct_name: String, content: &str) -> Option<String> {
    let use_statements = extract_use_statements(&content);
    let parts: Vec<&str> = struct_name.split("::").collect();
    let mut use_statement = String::new();
    let struct_name: String;

    if parts.len() > 1 {
        struct_name = parts.last().unwrap().to_string();
        let other_parts = parts[..parts.len() - 1].join("::");
        use_statements.iter().for_each(|statement| {
            if statement.contains(parts.first().unwrap()) {
                use_statement = format!("{}::{}", statement, other_parts);
            }
        });
        if use_statement.is_empty() {
            use_statement = other_parts;
        }
    } else {
        struct_name = parts.first().unwrap().to_string();
        use_statements.iter().for_each(|statement| {
            if statement.contains(parts.first().unwrap()) {
                use_statement = statement.clone();
            }
        });
    }

    let parts: Vec<&str> = use_statement.split("::").collect();
    let mut struct_path: String;
    if parts.len() > 1 {
        struct_path = parts[..parts.len() - 1].join("/");
    } else {
        struct_path = parts.join("/");
    }

    struct_path = struct_path.replace("crate", "template"); //env!("CARGO_PKG_NAME")

    let doc_path = cargo_doc_path().join(format!("{}/struct.{}.html", struct_path, struct_name));

    if !doc_path.exists() {
        println!("Documentation file not found: {:?}", doc_path);
        return None;
    }

    let html_content = fs::read_to_string(&doc_path).ok()?;

    let mut ts_content = String::new();

    let struct_regex = Regex::new(r"struct\s+(\w+)\s*\{([^}]*)\}").expect("Invalid struct regex");
    for cap in struct_regex.captures_iter(&html_content) {
        let _struct_name = &cap[1];
        let fields = &cap[2];

        let ts_fields: Vec<String> = fields
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let re = Regex::new(r"<[^>]*>").unwrap();
                let html_text = re.replace_all(line, "").to_string();
                let parts: Vec<&str> = html_text.trim().split(':').collect();
                if parts.len() == 2 {
                    let field_name = parts[0].trim();

                    let field_type = rust_to_typescript_type(
                        parts[1].trim().trim_end_matches(','),
                        &vec![],
                        content,
                    );

                    format!("   {}: {};", field_name, field_type)
                } else {
                    String::new()
                }
            })
            .collect();

        ts_content.push_str(&format!(
            "{{\n{}\n}}",
            ts_fields.join("\n").replace("pub", "").trim()
        ));
    }

    Some(ts_content)
}

pub fn extract_use_statements(content: &str) -> Vec<String> {
    let mut use_statements = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            let statement = trimmed.strip_prefix("use ").unwrap().trim_end_matches(';');
            let expanded_statements = expand_use_statement(statement);
            use_statements.extend(expanded_statements);
        }
    }

    use_statements
}

fn expand_use_statement(statement: &str) -> Vec<String> {
    let mut results = Vec::new();

    if let Some((prefix, grouped)) = statement.split_once("::{") {
        let grouped = grouped.trim_end_matches('}');
        let imports = split_imports(grouped);
        for import in imports {
            results.push(format!("use {}::{};", prefix, import));
        }
    } else if let Some((prefix, grouped)) = statement.split_once('{') {
        let prefix = prefix.trim_end();
        let grouped = grouped.trim_end_matches('}');
        let imports = split_imports(grouped);
        for import in imports {
            results.push(format!("use {}{};", prefix, import));
        }
    } else {
        results.push(format!("use {};", statement));
    }

    results
}

fn split_imports(grouped: &str) -> Vec<String> {
    grouped.split(',').map(|s| s.trim().to_string()).collect()
}
