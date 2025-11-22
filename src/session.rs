pub mod manager;

use crate::tmux;
use std::path::PathBuf;

pub fn create_or_attach(name: &str, dir: Option<&PathBuf>, detached: bool) -> Result<(), String> {
    if tmux::session_exists(name)? {
        #[cfg(debug_assertions)]
        println!("Session '{}' already exists, attaching...", name);
        attach(name)
    } else {
        #[cfg(debug_assertions)]
        println!("Creating new session '{}'...", name);
        create(name, dir, detached)
    }
}

pub fn create(name: &str, dir: Option<&PathBuf>, detached: bool) -> Result<(), String> {
    if let Some(path) = &dir {
        if !path.exists() {
            return Err(format!("Directory '{}' does not exist", path.display()));
        }
    }

    tmux::create_session(name, dir, detached)?;

    if !detached {
        #[cfg(debug_assertions)]
        println!("Session '{}' created successfully", name);
    }

    Ok(())
}

pub fn attach(name: &str) -> Result<(), String> {
    if !tmux::session_exists(name)? {
        return Err(format!("Session '{}' does not exist", name));
    }

    tmux::attach_to_session(name)?;
    Ok(())
}

pub fn list() -> Result<(), String> {
    let sessions = tmux::list_sessions()?;

    if sessions.is_empty() {
        #[cfg(debug_assertions)]
        println!("No tmux sessions found");
    } else {
        #[cfg(debug_assertions)]
        println!("Active tmux sessions:");
        for item in sessions {
            println!("- {}", item);
        }
    }

    Ok(())
}

pub fn kill(name: &str) -> Result<(), String> {
    if !tmux::session_exists(name)? {
        return Err(format!("Session '{}' does not exist", name));
    }

    tmux::kill_session(name)?;
    println!("Session '{}' killed successfully", name);
    Ok(())
}
