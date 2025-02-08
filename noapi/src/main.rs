mod boilerplate;
mod cargo_commands;
mod js_commands;
mod noapi_config;

use cargo_commands::{
    cargo_build, cargo_check_installed, cargo_doc, cargo_install, is_valid_cargo_name,
    run_start_command,
};
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::Select;
use js_commands::{bun_install, deno_install, npm_install, pnpm_install, yarn_install};
use serde::{Deserialize, Serialize};
use std::{fs, io::Read, path::Path};

#[derive(Parser, Debug)]
#[command(
    name = "noapi",
    version = "0.1.4",
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

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum JsRuntime {
    Deno,
    Bun,
    Node,
}

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Deno,
    Bun,
    NPM,
    PNPM,
    YARN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    js_runtime: JsRuntime,
    package_manager: PackageManager,
}

impl Config {
    fn new() -> Self {
        let runtime_options = ["Deno", "Bun", "Node"];
        let runtime_selection = Select::new()
            .with_prompt("Select a JavaScript runtime")
            .items(&runtime_options)
            .default(0)
            .interact()
            .unwrap();

        let js_runtime = match runtime_selection {
            0 => JsRuntime::Deno,
            1 => JsRuntime::Bun,
            _ => JsRuntime::Node,
        };

        let package_manager = match js_runtime {
            JsRuntime::Deno => PackageManager::Deno,
            JsRuntime::Bun => PackageManager::Bun,
            JsRuntime::Node => {
                let managers = ["npm", "yarn", "pnpm"];
                let selection = Select::new()
                    .with_prompt("Select a package manager")
                    .items(&managers)
                    .default(0)
                    .interact()
                    .unwrap();
                match selection {
                    0 => PackageManager::NPM,
                    1 => PackageManager::YARN,
                    _ => PackageManager::PNPM,
                }
            }
        };

        Self {
            js_runtime,
            package_manager,
        }
    }

    fn to_string(&self) -> String {
        toml::to_string_pretty(self).expect("Failed to serialize config")
    }

    /*fn save_to_file(&self) {
        let config_path = Path::new("config.toml");
        let toml_str = toml::to_string_pretty(self).expect("Failed to serialize config");
        fs::write(&config_path, toml_str).expect("Failed to write config file");
    }*/

    fn from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");
        let mut file = fs::File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { project_name } => {
            let config = Config::new();
            if is_valid_cargo_name(project_name) {
                if let Err(e) = boilerplate::generate_boilerplate(project_name, config.to_string())
                {
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
