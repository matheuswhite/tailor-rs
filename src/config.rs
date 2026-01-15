use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub registry_url: String,
}

impl Config {
    fn config_path() -> Result<PathBuf, String> {
        home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())
            .map(|home| home.join(".tailor").join("config.toml"))
    }

    pub fn create_default_config() -> Result<(), String> {
        let config = Config::default();
        let config_file = Self::config_path()?;

        if !config_file.exists() {
            let config_dir = config_file
                .parent()
                .ok_or_else(|| "Failed to get config parent directory".to_string())?;

            std::fs::create_dir_all(config_dir)
                .map_err(|_| "Failed to create config parent directory".to_string())?;

            std::fs::write(
                &config_file,
                toml::to_string_pretty(&config)
                    .map_err(|_| "Failed to serialize config".to_string())?,
            )
            .map_err(|_| "Failed to create config file".to_string())?;
        }

        Ok(())
    }

    pub fn load() -> Result<Self, String> {
        let config_file = Self::config_path()?;

        let config_content = std::fs::read_to_string(&config_file)
            .map_err(|_| "Failed to read config file".to_string())?;
        toml::from_str(&config_content).map_err(|_| "Failed to parse config file".to_string())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            registry_url: "https://registry.tailor.rs".to_string(),
        }
    }
}
