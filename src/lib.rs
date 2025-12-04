use std::{error::Error, path::PathBuf};

use clap::Parser;

use crate::cli::Cli;

mod cli;
mod config;
mod dir;
mod session;
mod source;
mod tmux;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let config = config::Config::load_or_default(
        &cli.config.unwrap_or_else(|| config::default_config_path()),
    );

    match &cli.session {
        Some(session) => {
            let dir = PathBuf::from(&session);
            let name = dir.file_name().unwrap().to_str().unwrap().replace('.', "_");

            if tmux::session_exists(&name)? {
                tmux::switch_session(&name)?;
            } else {
                tmux::new_session(&name, &dir)?;
            }
        }
        None => source::gather_sources(&config).map(|sources| {
            for source in sources {
                println!("{}", source);
            }
        })?,
    }

    Ok(())
}
