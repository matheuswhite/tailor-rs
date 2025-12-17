use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub registry_url: String,
}

impl Config {
    fn config_path() -> PathBuf {
        home_dir()
            .expect("Failed to get home directory")
            .join(".tailor")
            .join("config.toml")
    }

    pub fn create_default_config() {
        let config = Config::default();
        let config_file = Self::config_path();

        if !config_file.exists() {
            std::fs::create_dir_all(config_file.parent().unwrap())
                .expect("Failed to create config directory");

            std::fs::write(&config_file, toml::to_string_pretty(&config).unwrap())
                .expect("Failed to create config file");
        }
    }

    pub fn load() -> Self {
        let config_file = Self::config_path();

        let config_content =
            std::fs::read_to_string(&config_file).expect("Failed to read config file");

        toml::from_str(&config_content).expect("Failed to parse config file")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            registry_url: "https://registry.tailor.rs".to_string(),
        }
    }
}
