pub mod path {
    use std::path::{Path, PathBuf};

    pub fn expand_tilde(path: &Path) -> PathBuf {
        if path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                return home.join(path.strip_prefix("~").unwrap());
            }
        }
        path.to_path_buf()
    }

    pub fn collapse_tilde(path: &Path) -> String {
        if let Some(home) = dirs::home_dir() {
            if let Ok(stripped) = path.strip_prefix(&home) {
                return format!("~/{}", stripped.display());
            }
        }
        path.display().to_string()
    }
}
