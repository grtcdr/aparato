use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

/// This function returns a list of entries located inside a given directory.
#[doc(hidden)]
pub fn list_dir_entries(path: &str) -> Vec<PathBuf> {
    let mut directory_entries: Vec<PathBuf> = Vec::new();
    let directory = std::fs::read_dir(path);

    if let Ok(dir) = directory {
        for entry in dir.flatten() {
            directory_entries.push(entry.path())
        }
    }
    directory_entries
}

/// This function returns the basename of a given path.
#[doc(hidden)]
pub fn basename<'a>(path: String) -> String {
    let mut pieces = path.rsplit("/");
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

#[allow(dead_code)]
/// This function returns an iterator over the lines of a given file.
#[doc(hidden)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
