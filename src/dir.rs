use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct DirectoryConfig(pub PathBuf, pub usize);

impl Serialize for DirectoryConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let path_str = contract_path(&self.0);
        if self.1 > 0 {
            serializer.serialize_str(&format!("{}:{}", path_str, self.1))
        } else {
            serializer.serialize_str(&path_str)
        }
    }
}

impl<'de> Deserialize<'de> for DirectoryConfig {
    fn deserialize<D>(deserializer: D) -> Result<DirectoryConfig, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if let Some((path_part, depth_part)) = s.rsplit_once(':') {
            let path = expand_path(path_part);
            let depth = depth_part.parse::<usize>().unwrap();
            Ok(DirectoryConfig(path, depth))
        } else {
            let path = expand_path(&s);
            Ok(DirectoryConfig(path, 1))
        }
    }
}

pub fn list_directories(
    base_dirs: &[DirectoryConfig],
    show_hidden: bool,
    follow_symlinks: bool,
) -> Vec<PathBuf> {
    let mut dirs = vec![];

    for base in base_dirs {
        WalkDir::new(&base.0)
            .max_depth(base.1)
            .min_depth(0)
            .follow_links(follow_symlinks)
            .into_iter()
            .filter_map(|i| i.ok())
            .filter(|i| i.path().is_dir())
            .filter(|i| {
                if show_hidden {
                    true
                } else {
                    i.path()
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(|name_str| !name_str.starts_with('.'))
                        .unwrap_or(false)
                }
            })
            .for_each(|i| dirs.push(i.path().to_path_buf()));
    }

    dirs.sort();
    dirs.dedup();

    dirs
}

pub fn expand_path(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from(path))
            .join(stripped)
    } else if path == "~" {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(path))
    } else {
        PathBuf::from(path)
    }
}

pub fn contract_path(path: &Path) -> String {
    if let Some(home) = dirs::home_dir()
        && path.starts_with(&home)
    {
        return path.to_str().unwrap().replace(home.to_str().unwrap(), "~");
    }

    path.to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use dirs::home_dir;
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_list_directories() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir_paths = vec![
            temp_dir.path().join("test/path/1"),
            temp_dir.path().join("test/path/2/another_path"),
        ];

        let base_dirs = &[
            DirectoryConfig(temp_dir.path().join("test"), 2),
            DirectoryConfig(temp_dir.path().join("test/path/2"), 1),
        ];

        for path in &test_dir_paths {
            fs::create_dir_all(path).unwrap();
        }

        let dir_list = list_directories(base_dirs, false, false);

        assert_eq!(
            dir_list,
            vec![
                temp_dir.path().join("test"),
                temp_dir.path().join("test/path"),
                temp_dir.path().join("test/path/1"),
                temp_dir.path().join("test/path/2"),
                temp_dir.path().join("test/path/2/another_path"),
            ]
        );
    }

    #[test]
    fn test_expand() {
        let path = "~/test/path";
        let stripped_path = path.strip_prefix("~/").unwrap();
        let home_dir = home_dir().unwrap();

        let expanded_path = expand_path(path);

        assert_eq!(expanded_path, home_dir.join(stripped_path))
    }
}
