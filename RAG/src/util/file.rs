use std::{env, fs::{self, File}, io::{Result, Write}, path::{self, PathBuf}};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileAlreadyExists(pub String);

pub fn create_file(bytes: &[u8], file_name: &str) -> Result<PathBuf>{
    let mut path = env::current_dir()?;
    path.push("files");
    match fs::create_dir(&path) {
        Ok(_) => debug!("Directory created at: {:?}", path),
        Err(_) => debug!("Directory {:?} already exists", path),
    }
    path.push(file_name);
    let mut file = File::create_new(&path)?;
    file.write(bytes)?;
    Ok(path)
}
