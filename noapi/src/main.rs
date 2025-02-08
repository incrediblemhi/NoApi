mod boilerplate;
mod cargo_commands;
mod config;
mod js_commands;

use cargo_commands::{
    cargo_build, cargo_check_installed, cargo_doc, cargo_install, is_valid_cargo_name,
    run_start_command,
};
use clap::{Parser, Subcommand};
use config::{Config, PackageManager};
use js_commands::{bun_install, deno_install, npm_install, pnpm_install, yarn_install};

#[derive(Parser, Debug)]
#[command(
    name = "noapi",
    version = "0.1.41",
    author = "Kelvin Osei",
    about = "A Rust fullstack web framework [Axum + React], with the concept of Rust Server Functions(RSFs), utilizes the type safety of Rust and TypeScript to make calls to server without APIs."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New { project_name: String },
    Install,
    RunServer,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { project_name } => {
            let config = Config::new();
            if is_valid_cargo_name(project_name) {
                if let Err(e) = boilerplate::generate_boilerplate(project_name, config) {
                    eprintln!("Error: {}", e);
                };
            } else {
                println!("❌ '{}' is NOT a valid NoApi project name!", project_name);
            }
        }
        Commands::Install => {
            if !cargo_check_installed("systemfd") {
                println!("Installing Systemfd...");
                cargo_install("systemfd");
            }
            if !cargo_check_installed("cargo-watch") {
                println!("Installing Cargo-watch...");
                cargo_install("cargo-watch");
            }
            let config = Config::from_file().unwrap();

            run_install_command(config.package_manager);
        }
        Commands::RunServer => {
            if !cargo_check_installed("systemfd") {
                println!("Installing Systemfd...");
                cargo_install("systemfd");
            }
            if !cargo_check_installed("cargo-watch") {
                println!("Installing Cargo-watch...");
                cargo_install("cargo-watch");
            }
            let config = Config::from_file().unwrap();
            run_install_command(config.package_manager);
            run_start_command();
        }
    }
}

pub fn run_install_command(js_package_manager: PackageManager) {
    match js_package_manager {
        PackageManager::Bun => {
            bun_install();
        }
        PackageManager::Deno => {
            deno_install();
        }
        PackageManager::NPM => {
            npm_install();
        }
        PackageManager::PNPM => {
            pnpm_install();
        }
        PackageManager::YARN => {
            yarn_install();
        }
    }
    npm_install();
    cargo_build();
    cargo_doc();
}
