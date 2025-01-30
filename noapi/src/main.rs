mod boilerplate;

use std::{path::Path, process::Command as StdCommand};

use clap::{arg, Command};

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

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
            Command::new("run")
            .short_flag('r')
                .about("Runs your project")
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("new") {
        if let Some(project_name) = sub_matches.get_one::<String>("project_name") {
            if let Err(e) = boilerplate::generate_boilerplate(project_name) {
                eprintln!("Error: {}", e);
            }
        }
    }

    if let Some(_run_matches) = matches.subcommand_matches("install") {
        run_install_command();
    }

    if let Some(_run_matches) = matches.subcommand_matches("run") {
        run_install_command();
        run_start_command();
    }
}

pub fn npm_install() {
    // check if npm is installed or else install npm
    if !Path::new("/node_modules").exists() {
        println!("Installing npm packages...");
        match StdCommand::new(NPM).arg("install").status() {
            Ok(_status) => {}
            Err(error) => {
                println!("{}", error)
            }
        };
    }
}

pub fn cargo_build() {
    match StdCommand::new("cargo").arg("build").status() {
        Ok(_status) => {}
        Err(error) => {
            println!("{}", error)
        }
    };
}

pub fn cargo_doc() {
    match StdCommand::new("cargo").arg("doc").status() {
        Ok(_status) => {}
        Err(error) => {
            println!("{}", error)
        }
    };
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
            println!("{}", error)
        }
    };
}
