use std::{fs::File, io::Read};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::absolute_path::AbsolutePath;

pub struct Checksum(pub [u8; 32]);

impl Checksum {
    fn from_file(path: &AbsolutePath) -> Result<Self, String> {
        let mut file =
            File::open(path.inner()).map_err(|err| format!("Failed to open file: {err}"))?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|err| format!("Failed to read file: {err}"))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let value = hasher.finalize().into();

        Ok(Self(value))
    }

    fn directory_manifest(root: &AbsolutePath) -> Result<Vec<(String, Checksum)>, String> {
        let mut entries = Vec::new();

        for entry in WalkDir::new(root.inner()).follow_links(false) {
            let entry = entry.map_err(|err| format!("Fail to get directory entry: {err}"))?;
            if entry.file_type().is_file() {
                let path = entry.path();

                let rel_path = path
                    .strip_prefix(root.inner())
                    .map_err(|err| {
                        format!(
                            "Failed to strip prefix '{}' from path '{}': {}",
                            root.inner().display(),
                            path.display(),
                            err
                        )
                    })?
                    .to_string_lossy()
                    .replace('\\', "/"); // normalize Windows → Unix

                let hash =
                    Self::from_file(&path.try_into().map_err(|err| {
                        format!("Failed to convert path to AbsolutePath: {}", err)
                    })?)?;
                entries.push((rel_path, hash));
            }
        }

        // Deterministic order
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(entries)
    }

    pub fn from_directory(root: &AbsolutePath) -> Result<Checksum, String> {
        let manifest = Self::directory_manifest(root)?;
        let mut hasher = Sha256::new();

        for (path, hash) in manifest {
            hasher.update(hex::encode(hash.0));
            hasher.update(b"  ");
            hasher.update(path.as_bytes());
            hasher.update(b"\n");
        }

        let value = hasher.finalize().into();

        Ok(Self(value))
    }
}
