use std::{error::Error, path::PathBuf};

use tmux_interface::{DisplayMessage, ListSessions, NewSession, SwitchClient, Tmux};

pub struct SessionManager {}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {}
    }

    pub fn current_session(&self) -> Result<Option<String>, Box<dyn Error>> {
        let output = Tmux::with_command(DisplayMessage::new().print().message("#S")).output()?;
        let session_name = String::from_utf8_lossy(&output.stdout()).trim().to_string();
        if session_name.is_empty() {
            Ok(None)
        } else {
            Ok(Some(session_name))
        }
    }

    pub fn list_sessions(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let output = Tmux::with_command(ListSessions::new().format("#{session_name}")).output()?;
        let sessions = String::from_utf8_lossy(&output.stdout())
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(sessions)
    }

    pub fn session_exists(&self, name: &String) -> Result<bool, Box<dyn Error>> {
        let sessions = self.list_sessions()?;
        Ok(sessions.contains(name))
    }

    pub fn attach_session(&self, dir: PathBuf, name: Option<String>) -> Result<(), Box<dyn Error>> {
        // Custom session name or get from directory
        let session_name: String = match name {
            Some(name) => name,
            None => dir
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .ok_or("Invalid directory name")?
                .to_string(),
        };
        let session_dir: String = dir.to_str().ok_or("Invalid directory path")?.to_string();

        // Does session exist
        if self.session_exists(&session_name)? {
            // If it does, switch to the session
            Tmux::with_command(SwitchClient::new().target_session(&session_name)).output()?;
        } else {
            // If not, create a new session
            Tmux::with_command(
                NewSession::new()
                    .session_name(&session_name)
                    .start_directory(&session_dir),
            )
            .output()?;
        }

        Ok(())
    }
}
