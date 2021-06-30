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

#[doc(hidden)]
pub fn autocomplete_path(path: &str) -> PathBuf {
    let mut path_vec = [path].to_vec();
    let pathbuf_vec = PathBuf::from(path_vec.concat());
    if !pathbuf_vec.is_dir() {
        path_vec.insert(0, "/sys/bus/pci/devices");
        if pathbuf_vec.is_dir() {
            return PathBuf::from(path_vec.concat());
        } else {
            let mut id = path.to_owned();
            id.insert_str(0, "0000:");
            std::mem::swap(&mut path_vec[1], &mut id.as_str());
            return PathBuf::from(path_vec.concat());
        }
    }
    
    return PathBuf::from(path_vec.concat());
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
