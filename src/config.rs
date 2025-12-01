use std::{error::Error, fmt::Display, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub directories: Vec<String>,
}

impl Config {
    fn from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        let config_content =
            std::fs::read_to_string(path).map_err(|_| ConfigError::FileNotFound)?;
        let config: Config = toml::from_str(&config_content).map_err(ConfigError::ParseError)?;
        Ok(config)
    }

    pub fn load_or_default(path: &PathBuf) -> Self {
        match Self::from_file(path) {
            Ok(config) => config,
            Err(_) => Self::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directories: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    ParseError(toml::de::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Config file not found"),
            ConfigError::ParseError(e) => write!(f, "Failed to parse config: {}", e),
        }
    }
}

impl Error for ConfigError {}

/// Default config file located at `$HOME/.config/tmux-sessionizer/config.toml`
pub fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("tmux-sessionizer")
        .join("config.toml")
}
