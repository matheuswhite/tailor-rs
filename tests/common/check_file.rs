use std::path::{Path, PathBuf};

pub struct CheckFile {
    path: PathBuf,
}

impl CheckFile {
    #[allow(unused)]
    pub fn assert_exists(&self) {
        assert!(
            self.path.exists() && self.path.is_file(),
            "Expected file {:?} to exist, but it does not.",
            self.path
        );
    }

    #[allow(unused)]
    pub fn assert(self, expected: &str) {
        assert!(
            self.path.exists(),
            "Expected file {:?} to exist, but it does not.",
            self.path
        );

        let content = std::fs::read_to_string(&self.path)
            .unwrap_or_else(|_| panic!("Failed to read file {:?}", self.path));

        assert_eq!(
            content, expected,
            "Content of file {:?} does not match expected content.",
            self.path
        );
    }
}

impl From<&Path> for CheckFile {
    fn from(value: &Path) -> Self {
        CheckFile {
            path: value.to_path_buf(),
        }
    }
}

impl From<PathBuf> for CheckFile {
    fn from(value: PathBuf) -> Self {
        CheckFile { path: value }
    }
}
