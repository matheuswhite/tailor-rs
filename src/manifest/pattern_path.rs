use std::fmt::{Display, Formatter};

use crate::absolute_path::{AbsolutePath, normalize_path_for_tools};

#[derive(Clone)]
pub struct PatternPath {
    base: AbsolutePath,
    pattern: String,
}

impl PatternPath {
    pub fn parse_paths(
        toml_table: &toml::Table,
        base: &AbsolutePath,
        key: &str,
        default: &str,
    ) -> Result<Vec<Self>, String> {
        let path_values = toml_table
            .get(key)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or(vec![toml::Value::String(default.to_string())]);

        let mut paths = vec![];
        for path_value in path_values {
            let path_str = path_value
                .as_str()
                .ok_or(format!("{} must be a string", key))?;
            paths.push(PatternPath {
                base: base.to_owned(),
                pattern: path_str.to_string(),
            });
        }

        Ok(paths)
    }
}

impl Display for PatternPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let abs_path = self.base.inner().join(&self.pattern);
        write!(f, "{}", normalize_path_for_tools(&abs_path))
    }
}
