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
pub enum Manifest {
    Package {
        name: String,
        version: String,
        type_: PackageType,
        tool: Option<Dependency>,
        dependencies: Vec<Dependency>,
        sources: Vec<PatternPath>,
        includes: Vec<PatternPath>,
    },
    Tool {
        name: String,
        version: String,
        script: String,
    },
}

impl Manifest {
    pub fn is_library(&self) -> bool {
        match self {
            Manifest::Package { type_, .. } => *type_ == PackageType::Library,
            Manifest::Tool { .. } => false,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Manifest::Package { name, version, .. } => format!("{}@{}", name, version),
            Manifest::Tool { name, version, .. } => format!("{}@{}", name, version),
        }
    }

    pub fn dependencies(&self) -> Vec<Dependency> {
        match self {
            Manifest::Package { dependencies, .. } => dependencies.clone(),
            Manifest::Tool { .. } => vec![],
        }
    }

    pub fn is_tool(&self) -> bool {
        matches!(self, Manifest::Tool { .. })
    }

    pub fn tool(&self) -> Option<Dependency> {
        match self {
            Manifest::Package { tool, .. } => tool.clone(),
            Manifest::Tool { .. } => None,
        }
    }

    pub fn set_tool(&mut self, tool_dependency: Dependency) {
        if let Manifest::Package { tool, .. } = self {
            *tool = Some(tool_dependency);
        }
    }

    pub fn sources(&self) -> Vec<String> {
        match self {
            Manifest::Package { sources, .. } => sources.iter().map(|p| p.to_string()).collect(),
            Manifest::Tool { .. } => vec![],
        }
    }

    pub fn includes(&self) -> Vec<String> {
        match self {
            Manifest::Package { includes, .. } => includes.iter().map(|p| p.to_string()).collect(),
            Manifest::Tool { .. } => vec![],
        }
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
        match type_ {
            "bin" | "lib" => {
                let dependencies = Dependency::parse_dependencies(&toml_table, location)
                    .map_err(|e| format!("Failed to parse dependencies: {}", e))?;
                let sources = PatternPath::parse_paths(&toml_table, location, "sources", "src/*.c")
                    .map_err(|e| format!("Failed to parse sources: {}", e))?;
                let includes =
                    PatternPath::parse_paths(&toml_table, location, "includes", "include/")
                        .map_err(|e| format!("Failed to parse includes: {}", e))?;

                Ok(Self::Package {
                    name,
                    version,
                    type_: if type_ == "bin" {
                        PackageType::Binary
                    } else {
                        PackageType::Library
                    },
                    tool: None,
                    dependencies,
                    sources,
                    includes,
                })
            }
            "tool" => {
                let script = Self::parse_script(&toml_table)?;

                Ok(Self::Tool {
                    name,
                    version,
                    script,
                })
            }
            other => Err(format!("Unknown manifest type: {}", other)),
        }
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

    fn parse_script(toml_table: &toml::Table) -> Result<String, String> {
        toml_table
            .get("script")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'script' field".to_string())
            .map(|s| s.to_string())
    }
}
