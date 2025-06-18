use sha2::{Digest, Sha256};
use std::path::PathBuf;
use toml::Value;

#[derive(Debug)]
pub enum Dependency {
    Registry {
        name: String,
        version: String,
    },
    Git {
        name: String,
        url: String,
        revision: String,
    },
    Local {
        #[allow(unused)]
        name: String,
        path: PathBuf,
    },
}

impl Dependency {
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        match self {
            Self::Registry { name, version } => {
                hasher.update(name.as_bytes());
                hasher.update(version.as_bytes());
            }
            Self::Git {
                name,
                url,
                revision,
            } => {
                hasher.update(name.as_bytes());
                hasher.update(url.as_bytes());
                hasher.update(revision.as_bytes());
            }
            Self::Local { name, path } => {
                hasher.update(name.as_bytes());
                hasher.update(path.to_string_lossy().as_bytes());
            }
        }
        hasher.finalize().to_vec()
    }

    pub fn from_content(name: &str, content: &Value) -> Result<Self, String> {
        [
            Self::parse_registry_string_dependency,
            Self::parse_registry_dependency,
            Self::parse_git_dependency,
            Self::parse_local_dependency,
        ]
        .iter()
        .filter_map(|parser| parser(name, content).ok())
        .next()
        .ok_or(format!(
            "Failed to parse dependency '{}' with content: {:?}",
            name, content
        ))
    }

    fn parse_registry_string_dependency(name: &str, content: &Value) -> Result<Self, String> {
        content
            .as_str()
            .map(|version| Self::Registry {
                name: name.to_string(),
                version: version.to_string(),
            })
            .ok_or_else(|| format!("Invalid registry dependency format for '{}'", name))
    }

    fn parse_registry_dependency(name: &str, content: &Value) -> Result<Self, String> {
        content
            .as_table()
            .and_then(|table| {
                Some(Self::Registry {
                    name: name.to_string(),
                    version: table
                        .get("version")
                        .and_then(Value::as_str)
                        .map(String::from)?,
                })
            })
            .ok_or_else(|| format!("Invalid registry dependency format for '{}'", name))
    }

    fn parse_git_dependency(name: &str, content: &Value) -> Result<Self, String> {
        content
            .as_table()
            .and_then(|table| {
                Some(Self::Git {
                    name: name.to_string(),
                    url: table.get("url").and_then(Value::as_str).map(String::from)?,
                    revision: table
                        .get("revision")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .unwrap_or_else(|| "main".to_string()),
                })
            })
            .ok_or_else(|| format!("Invalid git dependency format for '{}'", name))
    }

    fn parse_local_dependency(name: &str, content: &Value) -> Result<Self, String> {
        content
            .as_table()
            .and_then(|table| {
                Some(Self::Local {
                    name: name.to_string(),
                    path: table
                        .get("path")
                        .and_then(Value::as_str)
                        .map(PathBuf::from)?,
                })
            })
            .ok_or_else(|| format!("Invalid local dependency format for '{}'", name))
    }
}
