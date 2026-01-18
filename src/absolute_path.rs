use std::path::{Path, PathBuf};

#[derive(Clone, Default)]
pub struct AbsolutePath(PathBuf);

impl AbsolutePath {
    pub fn inner(&self) -> &Path {
        &self.0
    }

    pub fn join<P>(&self, path: P) -> AbsolutePath
    where
        P: AsRef<Path>,
    {
        AbsolutePath(self.0.join(path))
    }
}

pub fn normalize_path_for_tools(path: &Path) -> String {
    #[cfg(windows)]
    {
        let s = path.to_string_lossy();
        let s = s.as_ref();
        if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
            return format!(r"\\{}", rest);
        }
        if let Some(rest) = s.strip_prefix(r"\\?\") {
            return rest.to_string();
        }
        s.to_string()
    }
    #[cfg(not(windows))]
    {
        path.to_string_lossy().into_owned()
    }
}

impl TryFrom<&Path> for AbsolutePath {
    type Error = String;

    fn try_from(value: &Path) -> Result<Self, String> {
        if !value.exists() {
            std::fs::create_dir_all(value)
                .map_err(|err| format!("fail to create directory {value:?}: {}", err))?;
        }

        let abs_path = value
            .canonicalize()
            .map_err(|err| format!("fail to canonicalize path {value:?}: {}", err))?;

        Ok(AbsolutePath(abs_path))
    }
}

impl TryFrom<PathBuf> for AbsolutePath {
    type Error = String;

    fn try_from(value: PathBuf) -> Result<Self, String> {
        if !value.exists() {
            std::fs::create_dir_all(&value)
                .map_err(|err| format!("fail to create directory {:?}: {}", &value, err))?;
        }

        let abs_path = value
            .canonicalize()
            .map_err(|err| format!("fail to canonicalize path {value:?}: {}", err))?;

        Ok(AbsolutePath(abs_path))
    }
}
