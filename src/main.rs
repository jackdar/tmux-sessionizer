#![allow(dead_code)]
use std::path::PathBuf;

use clap::Parser;

use crate::{error::SessionizerError, picker::FzfPicker, tmux::TmuxManager};

pub mod config;
pub mod dir;
pub mod error;
pub mod picker;
pub mod session;
pub mod source;
pub mod tmux;

#[derive(Parser, Debug)]
#[command(
    name = "tmux-sessionizer",
    version = "0.1.0",
    author = "Jack Darlington <jackdarlington2001@gmail.com>",
    about = "A lightweight tmux session manager"
)]
pub struct Cli {
    /// Optional directory to attach or create a session directly
    pub session: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

fn main() -> Result<(), SessionizerError> {
    let cli = Cli::parse();

    let config = config::Config::try_from(cli.config)?;

    let session_manager = session::SessionManager {
        tmux_client: TmuxManager,
        picker: FzfPicker,
    };

    session_manager.run(cli.session, &config)
}
