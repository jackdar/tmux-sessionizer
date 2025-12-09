use std::{fmt::Display, path::PathBuf};

use crate::{
    config,
    dir::{self},
    session::{self, Session},
    tmux,
};

#[derive(Debug)]
pub enum Source {
    Session(Session),
    Directory(PathBuf),
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Session(session) => write!(f, "[*] {}", session),
            Self::Directory(dir) => write!(f, "{}", dir.display()),
        }
    }
}

pub fn gather_sources(config: &config::Config) -> Result<Vec<Source>, Box<dyn std::error::Error>> {
    let mut sources = Vec::new();

    sources.extend(
        tmux::list_sessions()
            .unwrap()
            .iter()
            .map(|s| Source::Session(session::Session::from_tmux(s))),
    );

    sources.extend(
        dir::list_directories(&config.directories)?
            .into_iter()
            .map(Source::Directory),
    );

    Ok(sources)
}
