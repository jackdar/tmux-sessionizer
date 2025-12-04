use std::path::{Path, PathBuf};
use std::{error::Error, fmt::Display};

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub directories: Vec<String>,
}

impl Config {
    pub fn load_or_default(path: &Path) -> Self {
        Self::from_file(path).unwrap_or_default()
    }

    fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let config_content =
            std::fs::read_to_string(path).map_err(|_| ConfigError::FileNotFound)?;
        let config = toml::from_str(&config_content).map_err(ConfigError::ParseError)?;
        Ok(config)
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
