use std::path::PathBuf;
use tmux_interface::{
    AttachSession, HasSession, KillSession, ListSessions, NewSession, SwitchClient, Tmux,
};

pub fn session_exists(name: &str) -> Result<bool, String> {
    Tmux::with_command(HasSession::new().target_session(name))
        .status()
        .map(|status| status.success())
        .map_err(|e| format!("Failed to check if session exists: {}", e))
}

pub fn create_session(name: &str, dir: Option<&PathBuf>, detached: bool) -> Result<(), String> {
    let inside_tmux = std::env::var("TMUX").is_ok();

    let mut new_session = NewSession::new().session_name(name);

    if let Some(path) = &dir {
        new_session = new_session.start_directory(path.to_str().unwrap());
    }

    // If inside tmux and we want to attach, always create detached then switch
    if inside_tmux && !detached {
        new_session = new_session.detached();
    } else if detached {
        new_session = new_session.detached();
    }

    Tmux::with_command(new_session)
        .status()
        .map_err(|e| format!("Failed to create session: {}", e))?;

    // Switch to the new session if we're inside tmux and want to attach
    if inside_tmux && !detached {
        Tmux::with_command(SwitchClient::new().target_session(name))
            .status()
            .map_err(|e| format!("Failed to switch to session: {}", e))?;
    }

    Ok(())
}

pub fn attach_to_session(name: &str) -> Result<(), String> {
    // Check if we're inside tmux
    if std::env::var("TMUX").is_ok() {
        // Use switch-client if already in tmux
        Tmux::with_command(SwitchClient::new().target_session(name))
            .status()
            .map_err(|e| format!("Failed to switch to session: {}", e))?;
    } else {
        // Use attach-session if not in tmux
        Tmux::with_command(AttachSession::new().target_session(name))
            .status()
            .map_err(|e| format!("Failed to attach to session: {}", e))?;
    }

    Ok(())
}

pub fn list_sessions() -> Result<Vec<String>, String> {
    let output = Tmux::with_command(ListSessions::new())
        .output()
        .map_err(|e| format!("Failed to list sessions: {}", e))?;

    // Iterate over each line and get the name of the session
    let sessions = String::from_utf8_lossy(&output.0.stdout)
        .lines()
        .filter_map(|line| line.split(':').next().map(|s| s.to_string()))
        .collect();

    Ok(sessions)
}

pub fn kill_session(name: &str) -> Result<(), String> {
    Tmux::with_command(KillSession::new().target_session(name))
        .output()
        .map_err(|e| format!("Failed to kill session: {}", e))?;

    Ok(())
}
