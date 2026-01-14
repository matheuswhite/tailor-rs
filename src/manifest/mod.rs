use crate::{
    absolute_path::AbsolutePath,
    manifest::{
        dependency::Dependency, edition::Edition, package_type::PackageType,
        pattern_path::PatternPath,
    },
};

pub mod dependency;
pub mod edition;
pub mod kv;
pub mod package_type;
pub mod pattern_path;

#[derive(Clone)]
pub struct Manifest {
    name: String,
    version: String,
    type_: PackageType,
    dependencies: Vec<Dependency>,
    sources: Vec<PatternPath>,
    includes: Vec<PatternPath>,
    compiler: String,
}

impl Manifest {
    pub fn is_library(&self) -> bool {
        self.type_ == PackageType::Library
    }

    pub fn full_name(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dependencies(&self) -> Vec<Dependency> {
        self.dependencies.clone()
    }

    fn resolve_source_pattern(pattern: &str) -> Vec<String> {
        // Expand "~/" to $HOME when present.
        let pattern = if let Some(rest) = pattern.strip_prefix("~/") {
            if let Ok(home) = std::env::var("HOME") {
                format!("{}/{}", home, rest)
            } else {
                pattern.to_string()
            }
        } else {
            pattern.to_string()
        };

        // Expand simple glob patterns (if any). If multiple matches exist, pick all patterns; if none, keep the original pattern.
        let has_glob_meta = pattern.contains('*') || pattern.contains('?') || pattern.contains('[');

        if has_glob_meta && let Ok(paths) = glob::glob(&pattern) {
            return paths
                .filter_map(Result::ok)
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>();
        }

        vec![pattern]
    }

    pub fn sources(&self) -> Vec<String> {
        self.sources
            .iter()
            .flat_map(|src| Self::resolve_source_pattern(src.to_string().as_str()))
            .collect()
    }

    pub fn includes(&self) -> &[PatternPath] {
        &self.includes
    }

    pub fn set_includes(&mut self, includes: Vec<PatternPath>) {
        self.includes = includes;
    }

    pub fn pkg_type(&self) -> PackageType {
        self.type_
    }

    pub fn compiler(&self) -> &str {
        &self.compiler
    }

    pub fn from_file(content: &str, location: &AbsolutePath) -> Result<Self, String> {
        let toml_table = content
            .parse::<toml::Table>()
            .map_err(|_| "The main manifest is not valid TOML Table".to_string())?;

        let name = Self::parse_name(&toml_table)?;
        let version = Self::parse_version(&toml_table)?;
        Edition::parse_edition(&toml_table)
            .map_err(|e| format!("Failed to parse edition: {}", e))?;

        let type_ = toml_table
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("bin");
        let dependencies = Dependency::parse_dependencies(&toml_table, location)
            .map_err(|e| format!("Failed to parse dependencies: {}", e))?;
        let sources = PatternPath::parse_paths(&toml_table, location, "sources", "src/*.c")
            .map_err(|e| format!("Failed to parse sources: {}", e))?;
        let includes = PatternPath::parse_paths(&toml_table, location, "includes", "include/")
            .map_err(|e| format!("Failed to parse includes: {}", e))?;
        let compiler = toml_table
            .get("compiler")
            .and_then(|v| v.as_str())
            .unwrap_or("gcc")
            .to_string();

        Ok(Self {
            name,
            version,
            type_: if type_ == "bin" {
                PackageType::Binary
            } else {
                PackageType::Library
            },
            dependencies,
            sources,
            includes,
            compiler,
        })
    }

    fn parse_name(toml_table: &toml::Table) -> Result<String, String> {
        toml_table
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' field".to_string())
            .map(|s| s.to_string())
    }

    fn parse_version(toml_table: &toml::Table) -> Result<String, String> {
        toml_table
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'version' field".to_string())
            .map(|s| s.to_string())
    }
}
