use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "tmux-sessionizer",
    version = "0.1.0",
    author = "Jack Darlington",
    about = "A lightweight tmux session manager"
)]
pub struct Cli {
    /// Optional directory to attach or create a session directly
    pub session: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}
