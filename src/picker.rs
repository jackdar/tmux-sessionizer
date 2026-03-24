use std::{
    fmt::Display,
    io::{self, Write},
    process::Stdio,
};

pub trait Picker: Default {
    type Error: std::error::Error + Send + Sync + 'static;

    fn pick<T>(&self, items: &[T]) -> Result<Option<String>, Self::Error>
    where
        T: Display + Clone;
}

#[derive(Debug, thiserror::Error)]
pub enum FzfError {
    #[error("fzf is not installed on this system")]
    NotInstalled(#[source] io::Error),

    #[error("fzf failed to spawn: {0}")]
    SpawnFailed(#[source] io::Error),

    #[error("fzf execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Failed to parse fzf selection: {0}")]
    ParseError(String),
}

#[derive(Debug, Default)]
pub struct FzfPicker;

impl FzfPicker {
    fn check_fzf_exists(&self) -> Result<(), FzfError> {
        std::process::Command::new("fzf")
            .arg("--version")
            .output()
            .map(|_| ())
            .map_err(FzfError::NotInstalled)
    }
}

impl Picker for FzfPicker {
    type Error = FzfError;

    fn pick<I>(&self, items: &[I]) -> Result<Option<String>, FzfError>
    where
        I: Display + Clone,
    {
        self.check_fzf_exists()?;

        let input = items
            .iter()
            .map(|item| item.to_string() + "\n")
            .collect::<String>();

        let mut child = std::process::Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(FzfError::SpawnFailed)?;

        {
            let stdin = child.stdin.as_mut().ok_or_else(|| {
                FzfError::SpawnFailed(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "Failed to get stdin handle",
                ))
            })?;

            stdin
                .write_all(input.as_bytes())
                .map_err(FzfError::SpawnFailed)?;
        }

        let output = child.wait_with_output().map_err(FzfError::SpawnFailed)?;
        if !output.status.success() {
            if matches!(output.status.code(), Some(1) | Some(130)) {
                return Ok(None);
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FzfError::ExecutionFailed(stderr.to_string()));
        }

        let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if selection.is_empty() {
            Ok(None)
        } else {
            Ok(Some(selection))
        }
    }
}
