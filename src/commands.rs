use std::{error::Error, fmt::Display, path::PathBuf};

use crate::config;

#[derive(Debug)]
pub enum CommandError {
    TmuxError(std::io::Error),
    NoCommandProvided,
    Other(String),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::TmuxError(e) => write!(f, "Tmux command failed: {}", e),
            CommandError::NoCommandProvided => write!(f, "No command provided"),
            CommandError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for CommandError {}

pub fn switch_session(name: &str) -> Result<(), CommandError> {
    println!("Switching to session: {}", name);
    if name.is_empty() {
        CommandError::Other("Session name cannot be empty".to_string());
    }

    // Get the final path segment, if it starts with a . replace with _
    let path = PathBuf::from(name)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('.', "_");

    std::process::Command::new("tmux")
        .arg("switch-client")
        .arg("-t")
        .arg(path)
        .status()
        .map_err(CommandError::TmuxError)?;
    Ok(())
}

pub fn list_sessions(config: &config::Config, exclude_current: bool) -> Result<(), CommandError> {
    if exclude_current {
        println!("Listing sessions excluding the current one.");
    } else {
        for dir in &config.directories {
            println!("{}", dir);
        }
    }
    Ok(())
}
