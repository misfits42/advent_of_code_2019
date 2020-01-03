/// This struct is used to represent a node in a processed orbit map. It has a set
/// name and can have any number of parents or children.
pub struct OrbitNode {
    id: String,
    children_id: Vec<String>,
    parent_id: String,
}

impl OrbitNode {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
            children_id: Vec::<String>::new(),
            parent_id: String::from(""),
        }
    }

    /// Gets the id of the OrbitNode.
    pub fn get_id(&self) -> String {
        return self.id.to_string();
    }

    /// Adds a new parent reference to the OrbitNode.
    pub fn add_parent_id(&mut self, parent: String) {
        self.parent_id = parent.clone();
    }

    /// Checks if the OrbitNode has at least one parent.
    pub fn has_parent(&self) -> bool {
        return !self.parent_id.is_empty();
    }

    /// Gets the names of the parents of the OrbitNode.
    pub fn get_parent_id(&self) -> String {
        return self.parent_id.clone();
    }

    /// Addsa a new children to the OrbitNode.
    pub fn add_child_id(&mut self, child: String) {
        self.children_id.push(child.clone());
    }

    pub fn get_children_ids(&self) -> Vec<String> {
        return self.children_id.to_vec();
    }
}
