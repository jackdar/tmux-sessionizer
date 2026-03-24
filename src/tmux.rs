use tmux_interface::{AttachSession, ListSessions, NewSession, SwitchClient, Tmux};

pub trait TmuxClient {
    fn inside_tmux(&self) -> bool;
    fn list_sessions(&self) -> Result<Vec<String>, TmuxError>;
    fn session_exists(&self, name: &str) -> Result<bool, TmuxError>;
    fn create_session(&self, name: &str, dir: &str, detached: bool) -> Result<(), TmuxError>;
    fn switch_session(&self, name: &str) -> Result<(), TmuxError>;
    fn attach_session(&self, name: &str) -> Result<(), TmuxError>;
}

#[derive(Debug, thiserror::Error)]
pub enum TmuxError {
    #[error("tmux command failed with status: {0}")]
    CommandFailed(std::process::ExitStatus),

    #[error("tmux execution error: {0}")]
    ExecutionError(String),

    #[error("tmux is not running in the current environment")]
    TmuxNotRunning,
}

#[derive(Debug, Default)]
pub struct TmuxManager;

impl TmuxClient for TmuxManager {
    fn inside_tmux(&self) -> bool {
        std::env::var("TMUX").is_ok()
    }

    fn switch_session(&self, name: &str) -> Result<(), TmuxError> {
        Tmux::with_command(SwitchClient::new().target_session(name)).check_status()
    }

    fn attach_session(&self, name: &str) -> Result<(), TmuxError> {
        Tmux::with_command(AttachSession::new().target_session(name)).check_status()
    }

    fn list_sessions(&self) -> Result<Vec<String>, TmuxError> {
        let mut output = vec![];

        let cmd = Tmux::with_command(ListSessions::new())
            .output()
            .map_err(|e| TmuxError::ExecutionError(e.to_string()))?;

        if !cmd.status().success() {
            if is_tmux_server_unavailable(cmd.status()) {
                return Ok(output);
            }

            return Err(TmuxError::CommandFailed(cmd.status()));
        }

        String::from_utf8_lossy(&cmd.stdout())
            .lines()
            .map(format_session_line)
            .for_each(|l| output.push(l));

        Ok(output)
    }

    fn session_exists(&self, name: &str) -> Result<bool, TmuxError> {
        Ok(self.list_sessions()?.iter().any(|s| s == name))
    }

    fn create_session(&self, name: &str, dir: &str, detached: bool) -> Result<(), TmuxError> {
        let cmd = if detached {
            NewSession::new()
                .session_name(name)
                .start_directory(dir)
                .detached()
        } else {
            NewSession::new().session_name(name).start_directory(dir)
        };

        Tmux::with_command(cmd).check_status()
    }
}

fn is_tmux_server_unavailable(status: std::process::ExitStatus) -> bool {
    status.code() == Some(1)
}

trait TmuxExt {
    fn check_status(self) -> Result<(), TmuxError>;
}

impl TmuxExt for Tmux<'_> {
    fn check_status(self) -> Result<(), TmuxError> {
        match self.status() {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => Err(TmuxError::CommandFailed(status)),
            Err(e) => Err(TmuxError::ExecutionError(e.to_string())),
        }
    }
}

fn format_session_line(line: &str) -> String {
    line.split(':')
        .next()
        .map(String::from)
        .unwrap_or_else(|| line.to_string())
}
