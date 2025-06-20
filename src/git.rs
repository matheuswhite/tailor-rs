use std::{
    path::Path,
    process::{Command, Stdio},
};

pub fn git_clone(url: &str, path: &Path) -> Result<(), String> {
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|_| ())
        .map_err(|e| format!("fail to clone repository: {}", e))
}

pub fn git_checkout(revision: &str, path: &Path) -> Result<(), String> {
    Command::new("git")
        .arg("checkout")
        .arg(revision)
        .current_dir(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|_| ())
        .map_err(|e| format!("fail to checkout revision: {}", e))
}
