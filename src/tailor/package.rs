use crate::dependency_tree::DependencyTree;
use crate::manifest::{Manifest, kv::KeyValue, pattern_path::PatternPath};
use crate::sources::registry::Registry;

pub struct Package {
    dep_tree: DependencyTree,
}

impl Package {
    pub fn load_from_manifest(manifest: Manifest, registry: &Registry) -> Result<Self, String> {
        let mut dep_tree = DependencyTree::resolve(manifest, registry)?;

        for (subtree, mfst) in dep_tree.dfs_iter_mut() {
            let includes = Self::resolve_includes(subtree)?;
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
        self.dep_tree
            .dfs_iter()
            .map(|(_, mfst)| mfst)
            .collect::<Vec<_>>()
    }

    pub fn manifest(&self) -> &Manifest {
        self.dep_tree.root()
    }

    fn resolve_includes(dep_tree: DependencyTree) -> Result<Vec<PatternPath>, String> {
        let mut includes = vec![];

        for (_, mfst) in dep_tree.dfs_iter() {
            includes.extend(mfst.includes().to_vec());
        }

        Ok(includes)
    }
}
