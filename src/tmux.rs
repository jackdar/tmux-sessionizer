use std::{error::Error, fmt::Display, path::Path, process::ExitStatus};

use tmux_interface::{ListSessions, NewSession, SwitchClient, Tmux};

#[derive(Debug)]
pub enum TmuxError {
    CommandFailed(ExitStatus),
    ExecutionError(String),
}

impl Display for TmuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandFailed(status) => write!(f, "Tmux command failed with status: {}", status),
            Self::ExecutionError(msg) => write!(f, "Tmux execution error: {}", msg),
        }
    }
}

impl Error for TmuxError {}

pub fn switch_session(name: &str) -> Result<(), TmuxError> {
    check_status(Tmux::with_command(SwitchClient::new().target_session(name)).status())
}

pub fn new_session(name: &str, dir: &Path) -> Result<(), TmuxError> {
    let dir_str = dir.to_str().ok_or_else(|| {
        TmuxError::ExecutionError("Invalid UTF-8 in directory path".to_string())
    })?;

    check_status(
        Tmux::with_command(
            NewSession::new()
                .session_name(name)
                .start_directory(dir_str)
                .detached(),
        )
        .status(),
    )?;

    switch_session(name)
}

pub fn session_exists(name: &str) -> Result<bool, TmuxError> {
    Ok(list_sessions()?.iter().any(|s| s == name))
}

pub fn list_sessions() -> Result<Vec<String>, TmuxError> {
    let output = Tmux::with_command(ListSessions::new())
        .output()
        .map_err(|e| TmuxError::ExecutionError(e.to_string()))?;

    if !output.status().success() {
        return Err(TmuxError::CommandFailed(output.status()));
    }

    Ok(String::from_utf8_lossy(&output.stdout())
        .lines()
        .map(String::from)
        .collect())
}

fn check_status(result: Result<ExitStatus, tmux_interface::Error>) -> Result<(), TmuxError> {
    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(TmuxError::CommandFailed(status)),
        Err(e) => Err(TmuxError::ExecutionError(e.to_string())),
    }
}
