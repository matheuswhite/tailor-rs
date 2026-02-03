use crate::{absolute_path::AbsolutePath, command::CommandIF, fmt::success, manifest::Manifest};

use clap::Args;
use std::path::{Path, PathBuf};

#[derive(Debug, Args)]
#[command(about = "Clean the build artifacts of the Tailor package located at the specified path.", long_about = None)]
pub struct CleanPkg {
    #[arg(
        help = "The path to the Tailor package to clean. If not provided, the current directory is used.",
        default_value = "."
    )]
    path: String,
}

impl CommandIF for CleanPkg {
    fn command(&self) -> Result<(), String> {
        let path: AbsolutePath = PathBuf::from(&self.path).try_into()?;
        let manifest_content = std::fs::read_to_string(path.inner().join("Tailor.toml"))
            .map_err(|_| "fail to read Tailor.toml".to_string())?;
        let _manifest = Manifest::from_file(&manifest_content, &path)?;

        let build_path = path.join("build").inner().to_owned();

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

impl CleanPkg {
    fn dir_size(path: &Path) -> u64 {
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

    fn count_dir_recursively(path: &Path) -> u64 {
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
