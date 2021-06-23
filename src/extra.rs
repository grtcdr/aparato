use std::path::{Path, PathBuf};

pub fn list_dir_entries(path: &Path) -> Vec<PathBuf> {
    let mut directory_entries: Vec<PathBuf> = Vec::new();
    let directory = std::fs::read_dir(path);

    if let Ok(dir) = directory {
        for entry in dir.flatten() {
            directory_entries.push(entry.path())
        }
    }
    directory_entries
}

pub fn basename<'a>(path: &'a str) -> PathBuf {
    let mut pieces = path.rsplit("/");
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}