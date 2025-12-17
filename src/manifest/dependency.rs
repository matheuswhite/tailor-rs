use crate::{
    absolute_path::AbsolutePath,
    manifest::kv::{KeyValue, Value},
    storage::Storage,
};

#[derive(Clone)]
pub enum Dependency {
    Registry {
        name: String,
        version: String,
        options: Vec<KeyValue>,
    },
    Git {
        name: String,
        url: String,
        revision: String,
        options: Vec<KeyValue>,
    },
    Local {
        name: String,
        path: AbsolutePath,
        options: Vec<KeyValue>,
    },
}

impl Dependency {
    pub fn options(&self) -> &[KeyValue] {
        match self {
            Dependency::Registry { options, .. } => options,
            Dependency::Git { options, .. } => options,
            Dependency::Local { options, .. } => options,
        }
    }

    pub fn parse_dependencies(
        toml_table: &toml::Table,
        location: &AbsolutePath,
    ) -> Result<Vec<Self>, String> {
        let mut dependencies = vec![];

        let Some(dependencies_table) = toml_table.get("dependencies").and_then(|v| v.as_table())
        else {
            return Ok(dependencies);
        };

        for (name, value) in dependencies_table {
            let dependency = Self::parse_single_dependency(name, value, toml_table, location)?;
            dependencies.push(dependency);
        }

        Ok(dependencies)
    }

    fn parse_options(name: &str, root: &toml::Table) -> Vec<KeyValue> {
        let mut options = vec![];
        let Some(dep_options_table) = root
            .get(name)
            .and_then(|v| v.as_table())
            .and_then(|table| table.get("options").and_then(|opts| opts.as_table()))
        else {
            return options;
        };

        for (opt_name, opt_value) in dep_options_table {
            let value = match opt_value {
                toml::Value::String(s) => Value::String(s.clone()),
                toml::Value::Float(f) => Value::Float(*f),
                toml::Value::Integer(i) => Value::Integer(*i),
                toml::Value::Boolean(b) => Value::Boolean(*b),
                _ => continue,
            };
            options.push(KeyValue {
                key: opt_name.clone(),
                value,
            });
        }

        options
    }

    fn parse_single_dependency(
        name: &str,
        value: &toml::Value,
        root: &toml::Table,
        location: &AbsolutePath,
    ) -> Result<Self, String> {
        if let Some(version) = value.as_str() {
            return Ok(Dependency::Registry {
                name: name.to_string(),
                version: version.to_string(),
                options: Self::parse_options(name, root),
            });
        }

        let Some(table) = value.as_table() else {
            return Err(format!(
                "Dependency '{}' has an invalid format; expected a string or a table",
                name
            ));
        };

        if let Some(url) = table.get("git").and_then(|v| v.as_str()) {
            let revision = table
                .get("rev")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or(format!("Dependency '{}' must have a revision", name))?;

            return Ok(Dependency::Git {
                name: name.to_string(),
                url: url.to_string(),
                revision,
                options: Self::parse_options(name, root),
            });
        }

        if let Some(path) = table.get("path").and_then(|v| v.as_str()) {
            return Ok(Dependency::Local {
                name: name.to_string(),
                path: location.join(path),
                options: Self::parse_options(name, root),
            });
        }

        Err(format!(
            "Dependency '{}' has an invalid format; expected 'git' or 'path' fields",
            name
        ))
    }
}

impl PartialEq for Dependency {
    fn eq(&self, other: &Self) -> bool {
        let my_storage_name = Storage::storage_name(self);
        let other_storage_name = Storage::storage_name(other);

        my_storage_name == other_storage_name
    }
}
