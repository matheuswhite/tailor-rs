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
            .expect("fail to get home directory")
            .join(".tailor")
            .join("packages")
            .try_into()
            .map_err(|err| format!("fail to get storage directory: {}", err))
    }

    pub fn storage_name(dependency: &Dependency) -> String {
        match dependency {
            Dependency::Registry { name, version, .. } => {
                format!("{}@{}", name, version)
            }
            Dependency::Git { name, revision, .. } => {
                format!("{}@{}", name, revision.clone())
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
            .map_err(|_| "fail to write checksum file".to_string())
    }

    fn load_manifest(storage_name: &AbsolutePath) -> Result<Manifest, String> {
        let manifest_path = storage_name.inner().join("Tailor.toml");
        let manifest_content = std::fs::read_to_string(manifest_path)
            .map_err(|_| "fail to read manifest from storage".to_string())?;
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
                .map_err(|_| "fail to remove corrupted dependency storage".to_string())?;
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
                    .map_err(|_| "fail to create local dependency storage".to_string())?;
                copy_dir_all(path.inner(), storage_name.inner()).map_err(|err| {
                    format!(
                        "fail to copy local dependency {} -> {}: {}",
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

fn copy_dir_all<P>(src: P, dst: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(&dst)?; // Ensure the destination directory exists

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.as_ref().join(entry.file_name());

        if ty.is_dir() {
            // Recursively call the function for subdirectories
            copy_dir_all(entry.path(), dest_path)?;
        } else {
            // Copy the file
            std::fs::copy(entry.path(), dest_path)?;
        }
    }

    Ok(())
}
