use std::fmt::Display;

#[derive(Debug)]
pub struct Session(String);

impl Session {
    pub fn from_tmux(line: &str) -> Self {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let name = parts[0].trim_end_matches(':').to_string();
        Self(name)
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
