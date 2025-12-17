use crate::{
    external_tool::registry::Registry,
    manifest::{Manifest, dependency::Dependency, kv::KeyValue, package_type::PackageType},
    storage::Storage,
};

pub struct Package {
    manifest: Manifest,
    dependencies: Vec<Manifest>,
}

impl Package {
    pub fn load_from_manifest(mut manifest: Manifest, registry: &Registry) -> Result<Self, String> {
        let mut open_list = manifest.dependencies().clone();
        let mut closed_list = vec![];
        let mut dependencies = vec![];

        while let Some(dependency) = open_list.pop() {
            if closed_list.contains(&dependency) {
                continue;
            }

            let dep_manifest = Storage::download(dependency.clone(), registry)?;
            if !dep_manifest.is_library() {
                return Err(format!(
                    "Dependency {} is not a library package",
                    dep_manifest.name()
                ));
            }

            closed_list.push(dependency.clone());
            dependencies.push(dep_manifest.clone());

            if dep_manifest.is_tool() {
                if manifest.tool().is_some() {
                    return Err("Multiple tools specified in dependencies".to_string());
                }

                manifest.set_tool(dependency);
            }

            for dep in dep_manifest.dependencies() {
                if !closed_list.contains(&dep) {
                    open_list.push(dep);
                }
            }
        }

        Ok(Self {
            manifest,
            dependencies,
        })
    }

    pub fn sources(&self) -> Vec<String> {
        let mut sources = self.manifest.sources();
        for dep in &self.dependencies {
            sources.extend(dep.sources());
        }
        sources
    }

    pub fn tool(&self) -> Option<Dependency> {
        self.manifest.tool()
    }

    pub fn includes(&self) -> Vec<String> {
        let mut includes = self.manifest.includes();
        for dep in &self.dependencies {
            includes.extend(dep.includes());
        }
        includes
    }

    pub fn pkg_type(&self) -> Result<PackageType, String> {
        match &self.manifest {
            Manifest::Package { type_, .. } => Ok(*type_),
            Manifest::Tool { .. } => Err("The manifest is for a tool, not a package".to_string()),
        }
    }

    pub fn name(&self) -> String {
        self.manifest.name()
    }

    pub fn options(&self) -> Vec<KeyValue> {
        match &self.manifest {
            Manifest::Package { dependencies, .. } => dependencies
                .iter()
                .flat_map(|dep| dep.options().to_vec())
                .collect::<Vec<_>>(),
            Manifest::Tool { .. } => vec![],
        }
    }
}
