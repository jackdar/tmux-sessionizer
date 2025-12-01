use std::error::Error;

use clap::Parser;

use crate::{cli::Cli, commands::CommandError};

mod cli;
mod commands;
mod config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Session name provided: {}", name);
    }

    let config_path = cli.config.unwrap_or_else(|| config::default_config_path());

    #[allow(unused_variables)]
    let config = config::Config::load_or_default(&config_path);

    match &cli.command {
        Some(cli::Commands::Switch { name }) => commands::switch_session(name)?,
        Some(cli::Commands::List { exclude_current }) => {
            commands::list_sessions(&config, *exclude_current)?
        }
        None => return Err(CommandError::NoCommandProvided.into()),
    }

    Ok(())
}
