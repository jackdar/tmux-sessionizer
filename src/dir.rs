use std::{fmt::Display, path::PathBuf};

use crate::config;

#[derive(Debug)]
pub struct Directory(PathBuf, u8);

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

fn parse_directory(input: &str) -> Directory {
    let parts: Vec<&str> = input.splitn(2, ':').collect();
    let path = expand(parts[0]);
    let depth = if parts.len() > 1 {
        parts[1].parse::<u8>().unwrap_or(0)
    } else {
        0
    };
    Directory(path, depth)
}

// fn list_directories(base_directories: &Vec<String>) -> Vec<Directory> {
//     let
// }

/// Expands `~` to the home directory and converts to an absolute path
pub fn expand(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        dirs::home_dir()
            .map(|home| home.join(stripped))
            .unwrap_or_else(|| PathBuf::from(path))
    } else if path == "~" {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(path))
    } else {
        PathBuf::from(path)
    }
}
