use crate::common::check_file::CheckFile;
use std::path::{Path, PathBuf};

pub struct CheckDir {
    path: PathBuf,
}

impl CheckDir {
    pub fn assert(&self) {
        assert!(
            self.path.exists() && self.path.is_dir(),
            "Expected directory {:?} to exist, but it does not.",
            self.path
        );
    }

    pub fn join(&self, segment: &str) -> CheckDir {
        CheckDir {
            path: self.path.join(segment),
        }
    }

    pub fn file(&self, segment: &str) -> CheckFile {
        CheckFile::from(self.path.join(segment))
    }
}

impl From<&Path> for CheckDir {
    fn from(value: &Path) -> Self {
        CheckDir {
            path: value.to_path_buf(),
        }
    }
}

impl From<PathBuf> for CheckDir {
    fn from(value: PathBuf) -> Self {
        CheckDir { path: value }
    }
}
