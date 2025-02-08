use serde::{Deserialize, Serialize};
use std::{fs, io::Read, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JsRuntime {
    Deno,
    Bun,
    Node,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn to_string(&self) -> String {
        toml::to_string_pretty(self).expect("Failed to serialize config")
    }

    /*fn save_to_file(&self) {
        let config_path = Path::new("config.toml");
        let toml_str = toml::to_string_pretty(self).expect("Failed to serialize config");
        fs::write(&config_path, toml_str).expect("Failed to write config file");
    }*/

    pub fn from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");
        let mut file = fs::File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
