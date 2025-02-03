mod boilerplate;
mod cargo_commands;

use cargo_commands::{cargo_build, cargo_doc, cargo_install};
use clap::{arg, Command};
use regex::Regex;
use std::{path::Path, process::Command as StdCommand};

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield", "union", "test", "rust",
];

fn main() {
    let matches = Command::new("noapi")
        .version("0.1.0")
        .author("Kelvin Osei")
        .about("A Rust fullstack web framework [Axum + React], with the concept of Rust server actions, utilizes the type safety of Rust and TypeScript to make calls to server without APIs.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
            .short_flag('n')
                .about("Creates a new project")
                .arg(arg!(project_name: "The name of your new project"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("install")
            .short_flag('i')
                .about("Installs dependencies")
        )
        .subcommand(
            Command::new("runserver")
            .short_flag('r')
                .about("Runs your project")
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("new") {
        if let Some(project_name) = sub_matches.get_one::<String>("project_name") {
            if is_valid_cargo_name(project_name) {
                if let Err(e) = boilerplate::generate_boilerplate(project_name) {
                    eprintln!("Error: {}", e);
                };
            } else {
                println!("❌ '{}' is NOT a valid NoApi project name!", project_name);
            }
        }
    }

    if let Some(_run_matches) = matches.subcommand_matches("install") {
        if !is_installed("systemfd") {
            println!("Installing Systemfd...");
            cargo_install("systemfd");
        }
        if !is_installed("cargo-watch") {
            println!("Installing Cargo-watch...");
            cargo_install("cargo-watch");
        }
        run_install_command();
    }

    if let Some(_run_matches) = matches.subcommand_matches("runserver") {
        if !is_installed("systemfd") {
            println!("Installing Systemfd...");
            cargo_install("systemfd");
        }
        if !is_installed("cargo-watch") {
            println!("Installing Cargo-watch...");
            cargo_install("cargo-watch");
        }
        run_install_command();
        run_start_command();
    }
}

fn is_installed(command: &str) -> bool {
    StdCommand::new(command)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn npm_install() {
    if !Path::new("/node_modules").exists() {
        println!("Installing npm packages...");
        match StdCommand::new(NPM).arg("install").status() {
            Ok(_status) => {}
            Err(error) => {
                eprintln!("{}", error)
            }
        };
    }
}

pub fn run_install_command() {
    npm_install();
    cargo_build();
    cargo_doc();
}

pub fn run_start_command() {
    match StdCommand::new("systemfd")
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

fn is_valid_cargo_name(name: &str) -> bool {
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
