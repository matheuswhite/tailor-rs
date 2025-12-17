use flate2::read::GzDecoder;
use std::{
    error::Error,
    path::{Path, PathBuf},
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

        let response = reqwest::blocking::get(url)?;
        let decoder = GzDecoder::new(response);
        let mut archive = Archive::new(decoder);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;

            // Remove o primeiro componente (repo-branch/)
            let stripped = path.components().skip(1).collect::<PathBuf>();

            // Ignora a pasta raiz
            if stripped.as_os_str().is_empty() {
                continue;
            }

            let out_path = dest.join(stripped);

            // Garante que o caminho não escape do diretório destino (segurança)
            if !Self::is_safe_path(dest, &out_path) {
                return Err("Caminho inseguro detectado no tar".into());
            }

            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            entry.unpack(out_path)?;
        }

        Ok(())
    }

    fn is_safe_path(base: &Path, target: &Path) -> bool {
        target.starts_with(base)
    }
}
