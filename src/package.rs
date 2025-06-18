use toml::Table;

use crate::dependency::Dependency;
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Default, Clone, Copy, Debug)]
pub enum PackageType {
    #[default]
    Binary,
    Library,
}

#[derive(Debug)]
pub struct Package {
    name: String,
    #[allow(unused)]
    version: String,
    dependencies: Vec<Dependency>,
    pkg_type: PackageType,
    sources: Vec<String>,
    includes: Vec<String>,
}

impl Package {
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.name.as_bytes());
        hasher.update(self.version.as_bytes());
        for dep in &self.dependencies {
            hasher.update(dep.hash());
        }
        for src in &self.sources {
            hasher.update(src.as_bytes());
        }
        for inc in &self.includes {
            hasher.update(inc.as_bytes());
        }
        hasher.finalize().to_vec()
    }

    pub fn dependencies(&self) -> &[Dependency] {
        &self.dependencies
    }

    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    pub fn includes(&self) -> &[String] {
        &self.includes
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pkg_type(&self) -> PackageType {
        self.pkg_type
    }

    pub fn from_file(filepath: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(filepath)
            .map_err(|e| format!("Failed to read file {}: {}", filepath.display(), e))?;

        Self::from_content(&content)
    }

    pub fn from_content(content: &str) -> Result<Self, String> {
        let parsed = content
            .parse::<Table>()
            .map_err(|_| format!("Invalid Toml format"))?;

        Ok(Package {
            name: parsed
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing 'name' field")?
                .to_string(),
            version: parsed
                .get("version")
                .and_then(|v| v.as_str())
                .ok_or("Missing 'version' field")?
                .to_string(),
            dependencies: parsed
                .get("dependencies")
                .and_then(|v| v.as_table())
                .map_or(vec![], |deps| {
                    deps.iter()
                        .filter_map(|(k, v)| Dependency::from_content(k, v).ok())
                        .collect()
                }),
            pkg_type: if parsed.get("lib").is_some() {
                PackageType::Library
            } else {
                PackageType::Binary
            },
            sources: parsed.get("src").and_then(|v| v.as_array()).map_or(
                vec!["src/*.c".to_string()],
                |arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                },
            ),
            includes: parsed.get("include").and_then(|v| v.as_array()).map_or(
                vec!["include/".to_string()],
                |arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                },
            ),
        })
    }
}
