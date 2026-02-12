use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct CompileCommandEntry {
    directory: PathBuf,
    arguments: Vec<String>,
    file: PathBuf,
}

impl CompileCommandEntry {
    pub fn new(directory: PathBuf, arguments: Vec<String>, file: PathBuf) -> Self {
        Self {
            directory,
            arguments,
            file,
        }
    }
}
