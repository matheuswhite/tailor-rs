pub struct DependencyTree<T>
where
    T: PartialEq,
{
    node: T,
    children: Vec<DependencyTree<T>>,
}

pub struct DfsIterator<'a, T>
where
    T: PartialEq,
{
    stack: Vec<&'a DependencyTree<T>>,
}

pub struct DfsIteratorMut<'a, T>
where
    T: PartialEq,
{
    stack: Vec<&'a mut DependencyTree<T>>,
}

impl<T> DependencyTree<T>
where
    T: PartialEq,
{
    pub fn resolve<F, A>(
        root: T,
        get_children: F,
        is_node_valid: A,
    ) -> Result<DependencyTree<T>, String>
    where
        F: Fn(&T) -> Result<Vec<T>, String> + Clone,
        A: Fn(&T) -> Result<(), String> + Clone,
    {
        let mut children = vec![];

        for child in get_children(&root)? {
            is_node_valid(&child)?;

            let child_tree = Self::resolve(child, get_children.clone(), is_node_valid.clone())?;
            children.push(child_tree);
        }

        Ok(DependencyTree {
            node: root,
            children,
        })
    }

    pub fn dfs_iter(&self) -> DfsIterator<'_, T> {
        DfsIterator { stack: vec![self] }
    }

    pub fn dfs_iter_mut(&mut self) -> DfsIteratorMut<'_, T> {
        DfsIteratorMut { stack: vec![self] }
    }
}

impl<T> PartialEq for DependencyTree<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl<'a, T> Iterator for DfsIterator<'a, T>
where
    T: PartialEq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        for child in current.children.iter().rev() {
            self.stack.push(child);
        }

        Some(&current.node)
    }
}

impl<'a, T> Iterator for DfsIteratorMut<'a, T>
where
    T: PartialEq,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        for child in current.children.iter_mut().rev() {
            self.stack.push(child);
        }

        Some(&mut current.node)
    }
}
