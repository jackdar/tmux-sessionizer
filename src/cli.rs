use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tmux-sessionizer",
    version = "0.1.0",
    author = "Jack Darlington",
    about = "A lightweight tmux session manager"
)]
pub struct Cli {
    /// Optional directory to attach or create a session directly
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Switch {
        /// Name of the session to switch to
        name: String,
    },
    List {
        /// Exclude the current session from the list
        #[arg(short, long)]
        exclude_current: bool,
    },
}
