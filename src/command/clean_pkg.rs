use crate::{absolute_path::AbsolutePath, command::Command, fmt::success, manifest::Manifest};
use std::path::PathBuf;

#[derive(Default)]
pub struct CleanPkg {
    path: AbsolutePath,
}

impl CleanPkg {
    fn dir_size(path: &PathBuf) -> u64 {
        let mut size = 0;

        let Ok(entries) = std::fs::read_dir(path) else {
            return size;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    size += metadata.len();
                }
            } else if path.is_dir() {
                size += Self::dir_size(&path);
            }
        }

        size
    }

    fn count_dir_recursively(path: &PathBuf) -> u64 {
        let mut count = 0;

        let Ok(entries) = std::fs::read_dir(path) else {
            return count;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                count += 1;
            } else if path.is_dir() {
                count += Self::count_dir_recursively(&path);
            }
        }

        count
    }

    fn to_fmt_bytes(bytes: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let bytes_f = bytes as f64;

        if bytes_f >= GB {
            format!("{:.1}GiB", bytes_f / GB)
        } else if bytes_f >= MB {
            format!("{:.1}MiB", bytes_f / MB)
        } else if bytes_f >= KB {
            format!("{:.1}KiB", bytes_f / KB)
        } else {
            format!("{}B", bytes)
        }
    }
}

impl Command for CleanPkg {
    fn parse_args(&mut self, args: &[String]) -> Option<()> {
        if args.is_empty() || args[0] != "clean" {
            return None;
        }

        match args.len() {
            1 => {
                self.path = std::env::current_dir().ok()?.try_into().ok()?;

                Some(())
            }
            2 => {
                self.path = PathBuf::from(&args[1]).try_into().ok()?;

                Some(())
            }
            _ => None,
        }
    }

    fn execute(&self) -> Result<(), String> {
        let manifest_content = std::fs::read_to_string(self.path.inner().join("Tailor.toml"))
            .map_err(|_| "fail to read Tailor.toml".to_string())?;
        let _manifest = Manifest::from_file(&manifest_content, &self.path)?;

        let build_path = self.path.join("build").inner().to_owned();

        let total_files = if build_path.exists() {
            Self::count_dir_recursively(&build_path)
        } else {
            0
        };

        let total_size = if build_path.exists() {
            Self::dir_size(&build_path)
        } else {
            0
        };

        if build_path.exists() {
            std::fs::remove_dir_all(&build_path)
                .map_err(|e| format!("Failed to clean build directory: {}", e))?;
        }

        println!(
            "{} {} file{}{}",
            success("Removed"),
            total_files,
            if total_files != 1 { "s" } else { "" },
            if total_size > 0 {
                format!(", {} total", Self::to_fmt_bytes(total_size))
            } else {
                "".to_string()
            }
        );

        Ok(())
    }
}
