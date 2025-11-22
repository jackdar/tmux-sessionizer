use std::{error::Error, path::PathBuf};

use jwalk::WalkDir;

use crate::{config::Config, session::manager::SessionManager, source::Source, utils::path};

pub struct SourceProvider<'a> {
    session_manager: &'a SessionManager,
    config: &'a Config,
}

impl<'a> SourceProvider<'a> {
    pub fn new(session_manager: &'a SessionManager, config: &'a Config) -> Self {
        SourceProvider {
            session_manager,
            config,
        }
    }

    /// Gathers sources for session creation based on configuration.
    pub fn gather_sources(&self) -> Result<Vec<Source>, Box<dyn Error>> {
        let mut sources = Vec::new();

        // If `exclude_current_session` is set in the configuration, retrieve the current session to exclude it later.
        let current = if self.config.exclude_current_session {
            self.session_manager.current_session()?
        } else {
            None
        };

        // Gather sessions
        sources.extend(
            self.session_manager
                .list_sessions()?
                .into_iter()
                .filter(|session| Some(session) != current.as_ref())
                .map(Source::TmuxSession),
        );

        // Gather directories
        sources.extend(
            self.gather_directories(
                self.config.directories.iter().map(PathBuf::from).collect(),
                self.config.max_depth,
            )
            .into_iter()
            .map(Source::Directory),
        );

        Ok(sources)
    }

    /// Gather directories up to a specified `max_depth` using jwalk
    fn gather_directories(&self, dirs: Vec<PathBuf>, max_depth: usize) -> Vec<PathBuf> {
        let mut collected = Vec::new();

        for dir in dirs {
            let expanded = path::expand_tilde(&dir);

            for entry in WalkDir::new(&expanded)
                .max_depth(max_depth)
                .skip_hidden(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_dir() {
                    collected.push(entry.path());
                }
            }
        }

        // Deduplicate: remove paths that are already included
        self.deduplicate_paths(collected)
    }

    /// Remove duplicate paths from the collected directories
    fn deduplicate_paths(&self, mut paths: Vec<PathBuf>) -> Vec<PathBuf> {
        // Sort to ensure consistent ordering
        // paths.sort();
        paths.dedup();
        paths
    }
}
