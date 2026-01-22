use crate::{external_tool::registry::Registry, manifest::Manifest, storage::Storage};

pub struct DependencyTree {
    manifest: Manifest,
    children: Vec<DependencyTree>,
}

pub struct DfsIterator<'a> {
    stack: Vec<&'a DependencyTree>,
}

pub struct DfsIteratorMut<'a> {
    stack: Vec<&'a mut DependencyTree>,
}

impl DependencyTree {
    pub fn resolve(manifest: Manifest, registry: &Registry) -> Result<DependencyTree, String> {
        let mut visited = vec![];
        Self::resolve_inernal(manifest, &mut visited, registry)
    }

    pub fn dfs_iter(&self) -> DfsIterator<'_> {
        DfsIterator { stack: vec![self] }
    }

    pub fn dfs_iter_mut(&mut self) -> DfsIteratorMut<'_> {
        DfsIteratorMut { stack: vec![self] }
    }

    pub fn root(&self) -> &Manifest {
        &self.manifest
    }

    fn get_children(manifest: &Manifest, registry: &Registry) -> Result<Vec<Manifest>, String> {
        let mut deps = vec![];

        for dep in manifest.dependencies() {
            let dep_manifest = Storage::download(dep.clone(), registry)?;
            deps.push(dep_manifest);
        }

        Ok(deps)
    }

    fn is_manifest_valid(manifest: &Manifest) -> Result<(), String> {
        if manifest.is_library() {
            Ok(())
        } else {
            Err(format!(
                "Dependency {} is not a library package",
                manifest.full_name()
            ))
        }
    }

    fn resolve_inernal(
        manifest: Manifest,
        visited: &mut Vec<Manifest>,
        registry: &Registry,
    ) -> Result<DependencyTree, String> {
        if visited.contains(&manifest) {
            return Err(format!(
                "Cyclic dependency detected for node {:?}",
                manifest
            ));
        }

        visited.push(manifest.clone());

        let mut children = vec![];

        for child in Self::get_children(&manifest, registry)? {
            Self::is_manifest_valid(&child)?;

            let child_tree = Self::resolve_inernal(child, visited, registry)?;
            children.push(child_tree);
        }

        Ok(DependencyTree { manifest, children })
    }
}

impl PartialEq for DependencyTree {
    fn eq(&self, other: &Self) -> bool {
        self.manifest == other.manifest
    }
}

impl<'a> Iterator for DfsIterator<'a> {
    type Item = &'a Manifest;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        for child in current.children.iter().rev() {
            self.stack.push(child);
        }

        Some(&current.manifest)
    }
}

impl<'a> Iterator for DfsIteratorMut<'a> {
    type Item = &'a mut Manifest;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        for child in current.children.iter_mut().rev() {
            self.stack.push(child);
        }

        Some(&mut current.manifest)
    }
}
