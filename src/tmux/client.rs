use std::{env, path::PathBuf};

use tmux_interface::{
    AttachSession, HasSession, KillSession, ListSessions, NewSession, SwitchClient, Tmux,
};

use crate::tmux::error::TmuxError;

pub struct TmuxClient;

impl Default for TmuxClient {
    fn default() -> Self {
        Self
    }
}

impl TmuxClient {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a tmux session exists with a given name
    pub fn session_exists(&self, name: &str) -> Result<bool, TmuxError> {
        Tmux::with_command(HasSession::new().target_session(name))
            .status()
            .map(|status| status.success())
            .map_err(|e| {
                TmuxError::CommandFailed(format!("Failed to check if session exists: {}", e))
            })
    }

    /// Create a new tmux session, if inside tmux, the session must be created then attached to
    pub fn create_session(
        &self,
        name: &str,
        dir: Option<&PathBuf>,
        detached: bool,
    ) -> Result<(), TmuxError> {
        let mut new_session = NewSession::new().session_name(name);

        if let Some(path) = &dir {
            new_session = new_session.start_directory(path.to_str().unwrap());
        }

        // If inside tmux and we want to attach, always create detached then switch
        if env::var("TMUX").is_ok() && !detached {
            new_session = new_session.detached();
        } else if detached {
            new_session = new_session.detached();
        }

        Tmux::with_command(new_session)
            .status()
            .map_err(|e| TmuxError::CommandFailed(format!("Failed to create session: {}", e)));

        // Switch to the new session if we're inside tmux and want to attach
        if env::var("TMUX").is_ok() && !detached {
            Tmux::with_command(SwitchClient::new().target_session(name))
                .status()
                .map_err(|e| {
                    TmuxError::CommandFailed(format!("Failed to switch to session: {}", e))
                });
        }

        Ok(())
    }

    /// Attach to a given tmux session, if inside tmux, use switch-client
    pub fn attach_to_session(&self, name: &str) -> Result<(), TmuxError> {
        // Check if we're inside tmux

        if env::var("TMUX").is_ok() {
            // Use switch-client if already in tmux
            Tmux::with_command(SwitchClient::new().target_session(name))
                .status()
                .map_err(|e| {
                    TmuxError::CommandFailed(format!("Failed to switch to session: {}", e))
                });
        } else {
            // Use attach-session if not in tmux
            Tmux::with_command(AttachSession::new().target_session(name))
                .status()
                .map_err(|e| {
                    TmuxError::CommandFailed(format!("Failed to attach to session: {}", e))
                });
        }

        Ok(())
    }

    /// List all current tmux sessions
    pub fn list_sessions(&self) -> Result<Vec<String>, TmuxError> {
        let output = Tmux::with_command(ListSessions::new())
            .output()
            .map_err(|e| TmuxError::CommandFailed(format!("Failed to list sessions: {}", e)));

        // Iterate over each line and get the name of the session
        let sessions = String::from_utf8_lossy(&output.unwrap().0.stdout)
            .lines()
            .filter_map(|line| line.split(':').next().map(|s| s.to_string()))
            .collect();

        Ok(sessions)
    }

    /// Kill a tmux session by name
    pub fn kill_session(&self, name: &str) -> Result<(), TmuxError> {
        Tmux::with_command(KillSession::new().target_session(name))
            .output()
            .map_err(|e| TmuxError::CommandFailed(format!("Failed to kill session: {}", e)));

        Ok(())
    }
}
