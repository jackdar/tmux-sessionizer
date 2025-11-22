#[allow(dead_code)]
use clap::Parser;
use std::error::Error;
use tmux_sessionizer::cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    tmux_sessionizer::run(args)?;

    Ok(())
}
