use flate2::read::GzDecoder;
use std::{
    error::Error,
    path::{Component, Path, PathBuf},
};
use tar::Archive;

use crate::absolute_path::AbsolutePath;

pub struct Git;

impl Git {
    pub fn clone_repository(url: &str, revision: &str, dest: &AbsolutePath) -> Result<(), String> {
        let url = format!("{}/archive/{}.tar.gz", url.trim_end_matches('/'), revision);

        Self::download_and_extract_flat(&url, dest.inner())
            .map_err(|err| format!("Failed to clone repository from {}: {}", url, err))
    }

    fn download_and_extract_flat(url: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(dest)?;
        let base = dest.canonicalize()?;

        let response = reqwest::blocking::get(url)?;
        let decoder = GzDecoder::new(response);
        let mut archive = Archive::new(decoder);

        for entry in archive.entries()? {
            let mut entry = entry?;

            let entry_type = entry.header().entry_type();
            if entry_type.is_symlink() || entry_type.is_hard_link() {
                return Err("Refusing to unpack symlink/hardlink entry from tar".into());
            }

            let path = entry.path()?;

            // Remove the first component (repo-branch/)
            let stripped = path.components().skip(1).collect::<PathBuf>();

            // Ignore the root folder
            if stripped.as_os_str().is_empty() {
                continue;
            }

            if !Self::is_valid_relative_path(&stripped) {
                return Err("Invalid path detected in tar (absolute path or '..')".into());
            }

            if Self::path_has_symlink(&base, &stripped)? {
                return Err(
                    "Refusing to unpack through symlink inside destination directory".into(),
                );
            }

            let out_path = base.join(&stripped);

            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;

                // Ensure the on-disk location of the parent folder is inside `base` (symlink-safe).
                if !Self::is_safe_path(&base, parent) {
                    return Err("Unsafe path detected in tar after canonicalization".into());
                }
            }

            entry.unpack(out_path)?;
        }

        Ok(())
    }

    fn is_safe_path(base: &Path, target: &Path) -> bool {
        let Ok(base) = base.canonicalize() else {
            return false;
        };
        let Ok(target) = target.canonicalize() else {
            return false;
        };
        target.starts_with(base)
    }

    fn is_valid_relative_path(path: &Path) -> bool {
        if path.is_absolute() {
            return false;
        }

        !path.components().any(|c| {
            matches!(
                c,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    }

    fn path_has_symlink(base: &Path, rel: &Path) -> Result<bool, std::io::Error> {
        let mut cur = base.to_path_buf();

        for comp in rel.components() {
            cur.push(comp);

            if let Ok(md) = std::fs::symlink_metadata(&cur)
                && md.file_type().is_symlink()
            {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
