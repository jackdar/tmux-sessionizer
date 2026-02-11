use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::dir::DirectoryConfig;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub directories: Vec<DirectoryConfig>,
    pub show_hidden: bool,
    pub follow_symlinks: bool,
    pub expand_tilde: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("config file does not exist at {0}")]
    ExistsError(PathBuf),

    #[error("failed to read config file at {0}: {1}")]
    IoError(PathBuf, #[source] std::io::Error),

    #[error("failed to parse config file at {0}: {1}")]
    ParseError(PathBuf, #[source] toml::de::Error),
}

impl TryFrom<Option<PathBuf>> for Config {
    type Error = ConfigError;

    fn try_from(value: Option<PathBuf>) -> Result<Self, Self::Error> {
        let path: PathBuf = match value {
            Some(v) if v.exists() => Ok(v),
            Some(v) => Err(ConfigError::ExistsError(v)),
            None => Ok(default_config_path()),
        }?;

        let config_content = fs::read_to_string(&path).unwrap_or(default_config_content());

        let config = toml::from_str(&config_content)
            .map_err(|e| ConfigError::ParseError(path.to_path_buf(), e))?;

        Ok(config)
    }
}

pub fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("tmux-sessionizer")
        .join("config.toml")
}

fn default_config_content() -> String {
    r#"
        directories = [
            "~:1",
        ]
        show_hidden = false
        follow_symlinks = false
        expand_tilde = false
        "#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config_content = r#"
            directories = [
                "/home/user/projects:2",
                "/home/user/workspace:3",
                "/home/user/test"
            ]
            show_hidden = false
            follow_symlinks = false
        "#;

        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(&config_path, config_content).unwrap();

        let config = Config::try_from(Some(config_path)).unwrap();

        assert_eq!(config.directories.len(), 3);

        assert_eq!(
            config.directories[0].0,
            PathBuf::from("/home/user/projects")
        );
        assert_eq!(config.directories[0].1, 2);

        assert_eq!(
            config.directories[1].0,
            PathBuf::from("/home/user/workspace")
        );
        assert_eq!(config.directories[1].1, 3);

        assert_eq!(config.directories[2].0, PathBuf::from("/home/user/test"));
        assert_eq!(config.directories[2].1, 1);
    }
}
