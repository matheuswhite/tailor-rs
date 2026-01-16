use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
    vec,
};

pub struct TailorUser;

impl TailorUser {
    fn tailor_binary() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("release")
            .join("tailor")
    }

    pub fn new_binary(&self, path: Option<&Path>, flag: bool) -> Output {
        let mut args = vec!["new"];

        if flag {
            args.push("--bin");
        }

        if let Some(p) = path {
            args.push(p.to_str().unwrap());
        }

        Command::new(TailorUser::tailor_binary())
            .args(&args)
            .output()
            .expect("Failed to execute tailor")
    }

    pub fn new_library(&self, path: Option<&Path>) -> Output {
        let mut args = vec!["new", "--lib"];

        if let Some(p) = path {
            args.push(p.to_str().unwrap());
        }

        Command::new(TailorUser::tailor_binary())
            .args(&args)
            .output()
            .expect("Failed to execute tailor")
    }
}

pub trait CheckOutput {
    fn assert_success(self, words: &[&str]);
    fn assert_failure(self, words: &[&str]);
}

impl CheckOutput for Output {
    fn assert_success(self, words: &[&str]) {
        assert!(
            self.status.success(),
            "Command failed with stderr: {}",
            String::from_utf8_lossy(&self.stderr)
        );

        let stdout = String::from_utf8_lossy(&self.stdout);
        for &word in words {
            assert!(
                stdout.contains(word),
                "Output should contain '{}', but it was not found.",
                word
            );
        }
    }

    fn assert_failure(self, words: &[&str]) {
        assert!(
            !self.status.success(),
            "Command succeeded unexpectedly. Stdout: {}",
            String::from_utf8_lossy(&self.stdout)
        );

        let stderr = String::from_utf8_lossy(&self.stderr);
        for &word in words {
            assert!(
                stderr.contains(word),
                "Error output should contain '{}', but it was not found.",
                word
            );
        }
    }
}
