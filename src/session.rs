use std::path::PathBuf;

use crate::{
    config::Config, dir, error::SessionizerError, picker::Picker, source::Source, tmux::TmuxClient,
};

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub dir: PathBuf,
}

impl TryFrom<PathBuf> for Session {
    type Error = SessionError;

    fn try_from(dir: PathBuf) -> Result<Self, Self::Error> {
        let raw = dir
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(SessionError::InvalidDirectoryName)?;

        Ok(Self {
            name: raw.replace('.', "_"),
            dir,
        })
    }
}

impl Session {
    fn dir_str(&self) -> Result<&str, SessionError> {
        self.dir
            .to_str()
            .ok_or_else(|| SessionError::NonUtf8DirectoryPath(self.dir.clone()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("could not derive session name from directory")]
    InvalidDirectoryName,

    #[error("session directory is not valid UTF-8: {0}")]
    NonUtf8DirectoryPath(PathBuf),
}

pub struct SessionManager<T: TmuxClient, P: Picker> {
    pub tmux_client: T,
    pub picker: P,
}

impl<T, P> SessionManager<T, P>
where
    T: TmuxClient,
    P: Picker,
{
    pub fn run(&self, arg: Option<PathBuf>, config: &Config) -> Result<(), SessionizerError> {
        if let Some(dir) = arg {
            let session = Session::try_from(dir)?;
            return self.create_or_switch_session(&session);
        }

        let sessions = self.tmux_client.list_sessions()?;
        let dirs = dir::list_directories(
            &config.directories,
            config.show_hidden,
            config.follow_symlinks,
        );

        let mut sources: Vec<Source> = Vec::with_capacity(sessions.len() + dirs.len());
        sources.extend(sessions.into_iter().map(Source::TmuxSession));
        sources.extend(dirs.into_iter().map(Source::Directory));

        let picker_items = sources
            .iter()
            .map(|source| source.to_picker_item(config.expand_tilde))
            .collect::<Vec<String>>();

        let Some(selection) = self
            .picker
            .pick(&picker_items)
            .map_err(|e| SessionizerError::Picker(Box::new(e)))
            .unwrap()
        else {
            return Ok(());
        };

        match Source::from(selection.as_str()) {
            Source::Directory(path) => {
                let session = Session::try_from(path)?;
                self.create_or_switch_session(&session)
            }
            Source::TmuxSession(name) => self.focus_existing_session(&name),
        }
    }

    pub fn create_or_switch_session(&self, session: &Session) -> Result<(), SessionizerError> {
        if self.tmux_client.session_exists(&session.name)? {
            self.focus_existing_session(&session.name)?;
        } else {
            if self.tmux_client.inside_tmux() {
                self.tmux_client
                    .create_session(&session.name, session.dir_str()?, true)?;
                self.tmux_client.switch_session(&session.name)?;
            } else {
                self.tmux_client
                    .create_session(&session.name, session.dir_str()?, false)?;
            }
        }

        Ok(())
    }

    fn focus_existing_session(&self, name: &str) -> Result<(), SessionizerError> {
        if self.tmux_client.inside_tmux() {
            self.tmux_client.switch_session(name)?;
        } else {
            self.tmux_client.attach_session(name)?;
        }

        Ok(())
    }
}
