use std::path::PathBuf;

use crate::utils::path;

pub mod provider;

#[derive(Clone, Debug)]
pub enum Source {
    TmuxSession(String),
    Directory(PathBuf),
}

impl Source {
    pub fn display_name(&self, display_tilde: bool) -> String {
        match self {
            Source::TmuxSession(name) => format!("[*] {}", name.clone()),
            Source::Directory(p) => {
                if display_tilde {
                    path::collapse_tilde(p)
                } else {
                    p.to_str().unwrap_or("unknown").to_string()
                }
            }
        }
    }

    /// Gather the session name to be used when interfacing with TMUX
    pub fn session_name(&self) -> String {
        // If the directory name starts with a dot, remove it
        match self {
            Source::TmuxSession(name) => name.clone(),
            Source::Directory(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.replacen(".", "_", 1))
                .unwrap_or("default_session".to_string()),
        }
    }
}
