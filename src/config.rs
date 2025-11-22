use std::{error::Error, fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub directories: Vec<String>,
    pub max_depth: usize,
    pub display_tilde: bool,
    pub exclude_current_session: bool,
    pub navigate_session: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            directories: vec![],
            max_depth: 1,
            display_tilde: true,
            exclude_current_session: true,
            navigate_session: true,
        }
    }
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}

pub fn default_config_path() -> PathBuf {
    let config_dir = dirs::home_dir().unwrap().join(".config");
    config_dir.join("tmux-sessionizer").join("config.toml")
}
