use super::node;

#[derive(Clone)]
pub struct Simulation {
    pub nodes: Vec<node::Node>,
    target_tps: u32,
    pub gravitational_constant: f32,
}

impl<'a> Simulation {
    pub fn new() -> Simulation {
        let nodes = Vec::new();
        Simulation {
            nodes,
            target_tps: 60,
            gravitational_constant: 1.0,
        }
    }

    pub fn add_node(&mut self, node: node::Node) {
        self.nodes.push(node)
    }

    pub fn remove_node(&mut self, id: node::Id) {
        self.nodes.retain(|node| node.id != id)
    }

    pub fn step(&mut self) {
        let nodes = self.nodes.clone();

        let others = nodes.clone();
        let node_force_function = |node: &mut node::Node| -> node::Force {
            let others = &others.iter().filter(|n| n != &node).collect();
            node::Force::calculate_incoming_force(node, others, &self.gravitational_constant)
        };

        for node in self.nodes.iter_mut() {
            node.step(node_force_function);
        }
    }

    pub fn handle_event(&mut self, event: node::Event) {
        match event.add_node_event {
            Some(event) => self.add_node(event.node),
            None => {}
        };
        match event.remove_node_event {
            Some(event) => self.remove_node(event.node_id),
            None => {}
        };
        match event.set_target_tps_event {
            Some(event) => match event.target_tps {
                Some(target_tps) => self.set_target_tps(target_tps),
                None => println!("TPS - {}", self.target_tps),
            },
            None => {}
        }
    }

    pub fn target_tps(&self) -> u32 {
        self.target_tps
    }

    pub fn set_target_tps(&mut self, target_tps: u32) {
        self.target_tps = target_tps;
    }
}

#[cfg(test)]
mod a_simulation {
    use crate::node;

    use super::*;

    #[test]
    fn has_no_nodes_on_creation() {
        let simulation = Simulation::new();
        assert!(simulation.nodes.is_empty());
    }

    #[test]
    fn can_add_node() {
        let mut simulation = Simulation::new();
        let id = node::Id(1);
        let position = node::Position(cgmath::Point3 {
            x: 3.0,
            y: 5.0,
            z: 7.0,
        });
        let node = node::Node::new(id, position);

        simulation.add_node(node.clone());

        assert_eq!(1, simulation.nodes.len());
        assert_eq!(&node, simulation.nodes.get(0).unwrap());
    }

    #[test]
    fn can_remove_node() {
        let mut simulation = Simulation::new();
        let id = node::Id(1);
        let position = node::Position(cgmath::Point3 {
            x: 3.0,
            y: 5.0,
            z: 7.0,
        });
        let node = node::Node::new(id, position);

        simulation.add_node(node.clone());

        simulation.remove_node(id);

        assert_eq!(0, simulation.nodes.len());
        assert!(!simulation.nodes.contains(&node));
    }
}
