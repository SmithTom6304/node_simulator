use super::node;

pub struct NodeCollection {
    nodes: Vec<node::Node>,
}

impl NodeCollection {
    pub fn new() -> NodeCollection {
        let nodes = Vec::new();
        NodeCollection { nodes }
    }

    pub fn add(&mut self, node: node::Node) {
        self.nodes.push(node);
    }

    pub fn remove(&mut self, id: node::NodeId) {
        self.nodes.retain(|node| node.id != id)
    }

    // pub fn find(&self, id: model::ModelId) -> Option<&model::Model> {
    //     self.iter().find(|model| model.id == id)
    // }

    // pub fn remove(&mut self, model_id: model::ModelId) -> bool {
    //     let size_before = self.models.len();
    //     self.models.retain(|model| model.id != model_id);
    //     self.models.len() != size_before
    // }

    pub fn iter(&self) -> std::slice::Iter<'_, node::Node> {
        self.nodes.iter()
    }
}
