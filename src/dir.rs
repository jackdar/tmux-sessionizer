use std::path::PathBuf;

#[derive(Debug)]
struct DirectoryConfig {
    path: PathBuf,
    depth: usize,
}

impl DirectoryConfig {
    pub fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.splitn(2, ':').collect();
        let path = expand(parts[0]);
        let depth = if parts.len() > 1 {
            parts[1].parse::<usize>().unwrap_or(0)
        } else {
            0
        };
        Self { path, depth }
    }
}

pub fn list_directories(
    base_directories: &[String],
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut dirs = Vec::new();

    for base in base_directories {
        let config = DirectoryConfig::from_str(&base);

        walkdir::WalkDir::new(&config.path)
            .max_depth(config.depth)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .for_each(|e| dirs.push(e.path().to_path_buf()));
    }

    Ok(dirs)
}

/// Expands `~` to the home directory and converts to an absolute path
pub fn expand(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        dirs::home_dir()
            .map(|home| home.join(stripped))
            .unwrap_or_else(|| PathBuf::from(path))
    } else if path == "~" {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(path))
    } else {
        PathBuf::from(path)
    }
}
