mod boilerplate;
mod cargo_commands;
mod js_commands;
mod noapi_config;

use cargo_commands::{
    cargo_build, cargo_check_installed, cargo_doc, cargo_install, is_valid_cargo_name,
    run_start_command,
};
use clap::{arg, Command};
use js_commands::npm_install;

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
        if !cargo_check_installed("systemfd") {
            println!("Installing Systemfd...");
            cargo_install("systemfd");
        }
        if !cargo_check_installed("cargo-watch") {
            println!("Installing Cargo-watch...");
            cargo_install("cargo-watch");
        }
        run_install_command();
    }

    if let Some(_run_matches) = matches.subcommand_matches("runserver") {
        if !cargo_check_installed("systemfd") {
            println!("Installing Systemfd...");
            cargo_install("systemfd");
        }
        if !cargo_check_installed("cargo-watch") {
            println!("Installing Cargo-watch...");
            cargo_install("cargo-watch");
        }
        run_install_command();
        run_start_command();
    }
}

pub fn run_install_command() {
    npm_install();
    cargo_build();
    cargo_doc();
}
