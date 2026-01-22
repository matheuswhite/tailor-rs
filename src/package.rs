use crate::{
    dependency_tree::DependencyTree,
    external_tool::registry::Registry,
    manifest::{Manifest, kv::KeyValue, pattern_path::PatternPath},
};

pub struct Package {
    dep_tree: DependencyTree,
}

impl Package {
    pub fn load_from_manifest(manifest: Manifest, registry: &Registry) -> Result<Self, String> {
        let mut dep_tree = DependencyTree::resolve(manifest, registry)?;

        for mfst in dep_tree.dfs_iter_mut() {
            let includes = Self::resolve_includes(mfst, registry)?;
            mfst.set_includes(includes);
        }

        Ok(Package { dep_tree })
    }

    pub fn options(&self) -> Vec<KeyValue> {
        self.dep_tree
            .root()
            .dependencies()
            .iter()
            .flat_map(|dep| dep.options().to_vec())
            .collect::<Vec<_>>()
    }

    pub fn manifests(&self) -> Vec<&Manifest> {
        self.dep_tree.dfs_iter().collect::<Vec<_>>()
    }

    pub fn manifest(&self) -> &Manifest {
        self.dep_tree.root()
    }

    fn resolve_includes(
        manifest: &Manifest,
        registry: &Registry,
    ) -> Result<Vec<PatternPath>, String> {
        let dep_tree = DependencyTree::resolve(manifest.clone(), registry)?;
        let mut includes = vec![];

        for mfst in dep_tree.dfs_iter() {
            includes.extend(mfst.includes().to_vec());
        }

        Ok(includes)
    }
}
