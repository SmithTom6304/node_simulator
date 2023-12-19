use super::node;

pub struct Simulation {
    pub nodes: Vec<node::Node>,
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
        self.nodes.retain(|node| node.id() != &id)
    }
}

#[cfg(test)]
mod a_simulation {
    use crate::node::{Node, NodeId, NodePosition};

    use super::*;

    #[test]
    fn has_no_nodes_on_creation() {
        let simulation = Simulation::new();
        assert!(simulation.nodes.is_empty());
    }

    #[test]
    fn can_add_node() {
        let mut simulation = Simulation::new();
        let id = NodeId(1);
        let position = NodePosition { x: 3, y: 5, z: 7 };
        let node = Node::new(id, position);

        simulation.add_node(node.clone());

        assert_eq!(1, simulation.nodes.len());
        assert_eq!(&node, simulation.nodes.get(0).unwrap());
    }

    #[test]
    fn can_remove_node() {
        let mut simulation = Simulation::new();
        let id = NodeId(1);
        let position = NodePosition { x: 3, y: 5, z: 7 };
        let node = Node::new(id, position);

        simulation.add_node(node.clone());

        simulation.remove_node(id);

        assert_eq!(0, simulation.nodes.len());
        assert!(!simulation.nodes.contains(&node));
    }
}
