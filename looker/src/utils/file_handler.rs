use project_root::get_project_root;
use std::env;
use std::fs::{File, canonicalize};
use std::path::{Path, PathBuf};

pub fn get_absolute_path(relative_path: &str) -> Result<PathBuf, std::io::Error> {
    let p_root = get_project_root()?;
    let full_path = p_root.join(relative_path);

    Ok(full_path)
}


pub fn load_file(path: &str) -> Result<File, std::io::Error> {
    let path = PathBuf::from(path);
    return File::open(&path);
}


pub(super) fn file_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_file()
}


fn get_argument(flag: &str) -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    for (index, argsv) in args.iter().enumerate() {
        if argsv == flag {
            match args.get(index + 1) {
                Some(argument) => return Ok(argument.to_string()),
                None => return Err(format!("Empty paramter for flag {}", flag)),
            }
        }
    }

    return Err(format!("Flag {} not found in args.", flag));
}