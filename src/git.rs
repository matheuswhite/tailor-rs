use std::{path::Path, process::Command};

pub fn git_clone(url: &str, path: &Path) -> Result<(), String> {
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .status()
        .map(|_| ())
        .map_err(|e| format!("Failed to clone repository: {}", e))
}

pub fn git_checkout(revision: &str, path: &Path) -> Result<(), String> {
    Command::new("git")
        .arg("checkout")
        .arg(revision)
        .current_dir(path)
        .status()
        .map(|_| ())
        .map_err(|e| format!("Failed to checkout revision: {}", e))
}
