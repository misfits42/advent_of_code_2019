/// This struct is used to represent a node in a processed orbit map. It has a set
/// name and can have any number of parents or children.
pub struct OrbitNode {
    id: String,
    children: Vec<String>,
    parent: Vec<String>,
}

impl OrbitNode {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
            children: Vec::<String>::new(),
            parent: Vec::<String>::new(),
        }
    }

    /// Gets the id of the OrbitNode.
    pub fn get_id(&self) -> String {
        return self.id.to_string();
    }

    /// Adds a new parent reference to the OrbitNode.
    pub fn add_parent(&mut self, parent: String) {
        self.parent.push(parent.clone());
    }

    /// Checks if the OrbitNode has at least one parent.
    pub fn has_parent(&self) -> bool {
        return !self.parent.is_empty();
    }

    /// Gets the names of the parents of the OrbitNode.
    pub fn get_parents(&self) -> Vec<String> {
        return self.parent.to_vec();
    }

    /// Addsa a new children to the OrbitNode.
    pub fn add_child(&mut self, child: String) {
        self.children.push(child.clone());
    }
}
