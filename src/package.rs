use crate::{
    dependency_tree::DependencyTree,
    external_tool::registry::Registry,
    manifest::{Manifest, kv::KeyValue, pattern_path::PatternPath},
    storage::Storage,
};

pub struct Package {
    manifest: Manifest,
    dependencies: Vec<Manifest>,
}

impl Package {
    fn dep_tree_of_manifest(
        manifest: &Manifest,
        registry: &Registry,
    ) -> Result<DependencyTree<Manifest>, String> {
        DependencyTree::resolve(
            manifest.clone(),
            |mfst| {
                let mut deps = vec![];

                for dep in mfst.dependencies() {
                    let dep_manifest = Storage::download(dep.clone(), registry)?;

                    deps.push(dep_manifest);
                }

                Ok(deps)
            },
            |mfst| {
                if mfst.is_library() {
                    Ok(())
                } else {
                    Err(format!(
                        "Dependency {} is not a library package",
                        mfst.full_name()
                    ))
                }
            },
        )
    }

    pub fn load_from_manifest(manifest: Manifest, registry: &Registry) -> Result<Self, String> {
        let mut dependency_tree = Self::dep_tree_of_manifest(&manifest, registry)?;

        for mfst in dependency_tree.dfs_iter_mut() {
            let includes = Self::resolve_includes(mfst, registry)?;
            mfst.set_includes(includes);
        }

        let mut iter = dependency_tree.dfs_iter();

        Ok(Package {
            manifest: iter.next().unwrap().clone(),
            dependencies: iter.cloned().collect(),
        })
    }

    fn resolve_includes(
        manifest: &Manifest,
        registry: &Registry,
    ) -> Result<Vec<PatternPath>, String> {
        let dep_tree = Self::dep_tree_of_manifest(manifest, registry)?;
        let mut includes = vec![];

        for mfst in dep_tree.dfs_iter() {
            includes.extend(mfst.includes().to_vec());
        }

        Ok(includes)
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
