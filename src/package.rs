use crate::{
    external_tool::registry::Registry,
    manifest::{Manifest, kv::KeyValue, pattern_path::PatternPath},
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
                    dep_manifest.full_name()
                ));
            }

            closed_list.push(dependency.clone());
            dependencies.push(dep_manifest.clone());

            for dep in dep_manifest.dependencies() {
                if !closed_list.contains(&dep) {
                    open_list.push(dep);
                }
            }
        }

        manifest.set_includes(Self::resolve_includes(&manifest));
        for dep in dependencies.iter_mut() {
            dep.set_includes(Self::resolve_includes(dep));
        }

        Ok(Self {
            manifest,
            dependencies,
        })
    }

    fn resolve_includes(manifest: &Manifest) -> Vec<PatternPath> {
        let mut includes = manifest.includes().to_vec();

        for dependency in manifest.dependencies() {
            let dep_manifest = Storage::download(dependency.clone(), &Registry::default())
                .expect("Failed to download dependency manifest");

            includes.extend(dep_manifest.includes().to_vec());
        }

        includes
    }

    pub fn options(&self) -> Vec<KeyValue> {
        self.manifest
            .dependencies()
            .iter()
            .flat_map(|dep| dep.options().to_vec())
            .collect::<Vec<_>>()
    }

    pub fn dependencies(&self) -> &[Manifest] {
        &self.dependencies
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}
