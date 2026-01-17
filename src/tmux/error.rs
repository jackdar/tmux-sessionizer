#[derive(Debug)]
pub enum TmuxError {
    SessionNotFound(String),
    CommandFailed(String),
}
