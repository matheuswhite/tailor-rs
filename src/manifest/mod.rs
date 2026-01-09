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
}

impl Manifest {
    pub fn is_library(&self) -> bool {
        self.type_ == PackageType::Library
    }

    pub fn name(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }

    pub fn dependencies(&self) -> Vec<Dependency> {
        self.dependencies.clone()
    }

    pub fn sources(&self) -> Vec<String> {
        self.sources.iter().map(|p| p.to_string()).collect()
    }

    pub fn includes(&self) -> Vec<String> {
        self.includes.iter().map(|p| p.to_string()).collect()
    }

    pub fn pkg_type(&self) -> PackageType {
        self.type_
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
