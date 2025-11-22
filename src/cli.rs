use clap::Parser;

/// Lightweight session manager for TMUX
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the session (used when no subcommand is provided)
    #[arg(value_name = "SESSION")]
    pub name: Option<String>,

    #[arg(short('c'), long("config"), value_name = "CONFIG_PATH", global = true)]
    pub config_path: Option<String>,
}
