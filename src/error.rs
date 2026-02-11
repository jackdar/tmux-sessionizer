use std::io;

use crate::{config::ConfigError, session::SessionError, tmux::TmuxError};

#[derive(Debug, thiserror::Error)]
pub enum SessionizerError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Session(#[from] SessionError),

    #[error(transparent)]
    Tmux(#[from] TmuxError),

    #[error(transparent)]
    Picker(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Io(#[from] io::Error),
}
