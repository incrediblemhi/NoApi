use clap::ValueEnum;
use dialoguer::Select;
use serde::{Deserialize, Serialize};
use std::{fs, io::Read, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum, PartialEq)]
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
pub struct Config {
    pub js_runtime: JsRuntime,
    pub package_manager: PackageManager,
}

impl Config {
    pub fn new() -> Self {
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

    pub fn to_string(&self) -> String {
        toml::to_string_pretty(self).expect("Failed to serialize config")
    }

    /*fn save_to_file(&self) {
        let config_path = Path::new("NoApi.toml");
        let toml_str = toml::to_string_pretty(self).expect("Failed to serialize config");
        fs::write(&config_path, toml_str).expect("Failed to write config file");
    }*/

    pub fn from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("NoApi.toml");
        let mut file = fs::File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
