use std::{fmt::Display, path::PathBuf};

use crate::dir::{contract_path, expand_path};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Source {
    Directory(PathBuf),
    TmuxSession(String),
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Directory(path) => write!(f, "{}", path.display()),
            Source::TmuxSession(name) => write!(f, "[*] {}", name),
        }
    }
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        Source::from(value.as_str())
    }
}

impl From<&str> for Source {
    fn from(value: &str) -> Self {
        if value.starts_with("[*] ") {
            Source::TmuxSession(value[4..].to_string())
        } else {
            Source::Directory(expand_path(value))
        }
    }
}

impl Source {
    pub fn to_picker_item(&self, expand_tilde: bool) -> String {
        match self {
            Source::Directory(path) => {
                if expand_tilde {
                    path.display().to_string()
                } else {
                    contract_path(path)
                }
            }
            Source::TmuxSession(name) => format!("[*] {}", name),
        }
    }
}
