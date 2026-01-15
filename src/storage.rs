use std::path::Path;

use crate::{
    absolute_path::AbsolutePath,
    external_tool::{checksum::Checksum, git::Git, registry::Registry},
    manifest::{Manifest, dependency::Dependency},
};
use dirs::home_dir;

pub struct Storage;

impl Storage {
    fn storage_dir() -> Result<AbsolutePath, String> {
        home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join(".tailor")
            .join("packages")
            .try_into()
            .map_err(|err| format!("Failed to get storage directory: {}", err))
    }

    pub fn storage_name(dependency: &Dependency) -> String {
        match dependency {
            Dependency::Registry { name, version, .. } => {
                format!("{}@{}", name, version)
            }
            Dependency::Git { name, revision, .. } => {
                format!("{}@{}", name, revision)
            }
            Dependency::Local { name, .. } => format!("{}@local", name),
        }
    }

    fn storage_name_path(dependency: &Dependency) -> Result<AbsolutePath, String> {
        let storage_dir = Self::storage_dir()?;
        let storage_name = Self::storage_name(dependency);

        storage_dir.inner().join(storage_name).try_into()
    }

    fn integrity(path: &AbsolutePath) -> bool {
        let Ok(checksum) = std::fs::read_to_string(path.inner().join("Tailor.sha256")) else {
            return false;
        };

        let Ok(expected_checksum) = hex::decode(checksum.trim()) else {
            return false;
        };

        let Ok(actual_checksum) = Checksum::from_directory(path) else {
            return false;
        };

        expected_checksum == actual_checksum.0
    }

    fn save_checksum(path: &AbsolutePath) -> Result<(), String> {
        let checksum = Checksum::from_directory(path)?;
        let checksum_hex = hex::encode(checksum.0);

        std::fs::write(path.inner().join("Tailor.sha256"), checksum_hex)
            .map_err(|_| "Failed to write checksum file".to_string())
    }

    fn load_manifest(storage_name: &AbsolutePath) -> Result<Manifest, String> {
        let manifest_path = storage_name.inner().join("Tailor.toml");
        let manifest_content = std::fs::read_to_string(manifest_path)
            .map_err(|_| "Failed to read manifest from storage".to_string())?;
        let manifest = Manifest::from_file(&manifest_content, storage_name)?;

        Ok(manifest)
    }

    pub fn download(dependency: Dependency, registry: &Registry) -> Result<Manifest, String> {
        let storage_name = Self::storage_name_path(&dependency)?;
        let exists = std::fs::metadata(storage_name.inner()).is_ok();
        let integrity = Self::integrity(&storage_name);

        if exists && integrity {
            return Self::load_manifest(&storage_name);
        }

        if exists && !integrity {
            std::fs::remove_dir_all(storage_name.inner())
                .map_err(|_| "Failed to remove corrupted dependency storage".to_string())?;
        }

        let manifest = match dependency {
            Dependency::Registry { name, version, .. } => {
                let package_url = registry.package_url(&name, &version)?;

                Git::clone_repository(&package_url, &version, &storage_name)?;

                Self::load_manifest(&storage_name)
            }
            Dependency::Git { url, revision, .. } => {
                Git::clone_repository(&url, &revision, &storage_name)?;

                Self::load_manifest(&storage_name)
            }
            Dependency::Local { path, .. } => {
                std::fs::create_dir_all(storage_name.inner())
                    .map_err(|_| "Failed to create local dependency storage".to_string())?;
                copy_dir_all(path.inner(), storage_name.inner()).map_err(|err| {
                    format!(
                        "Failed to copy local dependency {} -> {}: {}",
                        path.inner().display(),
                        storage_name.inner().display(),
                        err
                    )
                })?;

                Self::load_manifest(&storage_name)
            }
        };

        Self::save_checksum(&storage_name)?;

        manifest
    }
}

fn copy_dir_all<P, Q>(src: P, dst: Q) -> std::io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    std::fs::create_dir_all(&dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        // Safety: never follow symlinks when copying dependencies into storage.
        // Symlinks can lead to directory traversal or infinite loops.
        if file_type.is_symlink() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Refusing to copy symlink: {}", entry.path().display()),
            ));
        }

        let dest_path = dst.as_ref().join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(entry.path(), &dest_path)?;
        } else if file_type.is_file() {
            std::fs::copy(entry.path(), dest_path)?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Refusing to copy non-file entry: {}",
                    entry.path().display()
                ),
            ));
        }
    }

    Ok(())
}
