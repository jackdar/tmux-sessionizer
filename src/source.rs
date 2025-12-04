use std::fmt::Display;

use crate::{
    config,
    dir::{self, Directory},
    session::{self, Session},
    tmux,
};

#[derive(Debug)]
pub enum Source {
    Session(Session),
    Directory(Directory),
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Session(session) => write!(f, "[*] {}", session),
            Self::Directory(dir) => write!(f, "{}", dir),
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
        config
            .directories
            .iter()
            .map(|d| Source::Directory(dir::parse_directory(d))),
    );

    sources.extend(
        dir::list_directories(&config.directories)
            .iter()
            .map(Source::Directory),
    );

    Ok(sources)
}
