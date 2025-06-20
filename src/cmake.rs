use std::{path::Path, process::Command};

use crate::mode::Mode;

pub fn gen_cmake(mode: &Mode, path: &Path) -> Result<(), String> {
    Command::new("cmake")
        .arg("-S")
        .arg(path.join("build").join(mode.to_string()))
        .arg("-B")
        .arg(path.join("build").join(mode.to_string()))
        .arg(format!("-DCMAKE_BUILD_TYPE={}", mode.to_string()))
        .status()
        .map(|_| ())
        .map_err(|e| format!("fail to build: {}", e))
}

pub fn build(mode: &Mode, path: &Path) -> Result<(), String> {
    Command::new("cmake")
        .arg("--build")
        .arg(path.join("build").join(mode.to_string()))
        .status()
        .map(|_| ())
        .map_err(|e| format!("fail to run: {}", e))
}
