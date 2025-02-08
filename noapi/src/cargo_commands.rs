use std::process::Command;

use regex::Regex;

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield", "union", "test", "rust",
];

pub fn cargo_build() {
    match Command::new("cargo").arg("build").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn cargo_doc() {
    match Command::new("cargo").arg("doc").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn cargo_install(tool: &str) {
    match Command::new("cargo").arg("install").arg(tool).status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    }
}

pub fn run_start_command() {
    match Command::new("systemfd")
        .arg("--no-pid")
        .arg("-s")
        .arg("http::3000")
        .arg("--")
        .arg("cargo")
        .arg("watch")
        .arg("-x")
        .arg("run")
        .arg("-i")
        .arg("src/static")
        .arg("-i")
        .arg("node_modules")
        .arg("-i")
        .arg("functions.ts")
        .arg("-i")
        .arg("src/handlers")
        .status()
    {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn is_valid_cargo_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_-]*$").unwrap();

    if !re.is_match(name) {
        return false;
    }

    if name.chars().next().unwrap().is_numeric() {
        return false;
    }

    if RUST_KEYWORDS.contains(&name) {
        return false;
    }

    true
}

pub fn cargo_check_installed(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
