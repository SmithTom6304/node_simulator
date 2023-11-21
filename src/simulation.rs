use super::node;

pub struct Simulation {
    pub nodes: Vec<node::Node>,
    // Graphics
}

impl<'a> Simulation {
    pub fn new() -> Simulation {
        let nodes = Vec::new();
        Simulation { nodes }
    }

    pub fn add_node(&mut self, node: node::Node) {
        self.nodes.push(node)
    }

    pub fn remove_node(&mut self, id: node::NodeId) {
        self.nodes.retain(|node| node.id != id)
    }

    pub fn get_node(&'a self, id: node::NodeId) -> Option<&'a node::Node> {
        self.nodes.iter().find(|node| node.id == id)
    }
}
