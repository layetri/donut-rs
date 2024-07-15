use std::path::{Path, PathBuf};

pub fn default_path() -> PathBuf {
    homedir::my_home().unwrap().unwrap().join("donut")
}

pub fn get_wavs_in_path<T: AsRef<Path>>(path: T) -> Vec<PathBuf> {
    let files = std::fs::read_dir(path).unwrap();
    files.filter_map(|f| {
        let f = f.unwrap();
        let path = f.path();
        if path.extension().unwrap() == "wav" {
            Some(path)
        } else {
            None
        }
    }).collect()
}