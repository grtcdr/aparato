use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

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

// Returns the basename of a path
pub fn basename<'a>(path: String) -> String {
    let mut pieces = path.rsplit("/");
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

#[allow(dead_code)]
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
