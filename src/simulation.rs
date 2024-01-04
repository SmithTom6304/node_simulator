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
            gravitational_constant: -1.0,
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
        match event {
            node::Event::AddNode(add_node_event) => self.add_node(add_node_event.node),
            node::Event::RemoveNode(remove_node_event) => {
                self.remove_node(remove_node_event.node_id)
            }
            node::Event::SetTargetTps(set_target_tps_event) => {
                self.set_target_tps(set_target_tps_event.target_tps)
            }
            node::Event::SetNode(set_node_event) => {
                let node: &mut node::Node = match self
                    .nodes
                    .iter_mut()
                    .find(|node| node.id == set_node_event.id)
                {
                    Some(node) => node,
                    None => {
                        println!(
                            "No node with id {} was found",
                            set_node_event.id.to_string()
                        );
                        return;
                    }
                };

                match set_node_event.position {
                    Some(position) => node.position = position,
                    None => {}
                };
                match set_node_event.velocity {
                    Some(velocity) => node.velocity = velocity,
                    None => {}
                };
                match set_node_event.mass {
                    Some(mass) => node.mass = mass,
                    None => {}
                };
                match set_node_event.gravitational_constant_override {
                    Some(g) => node.gravitational_constant_override = Some(g),
                    None => {}
                };
                match set_node_event.dampen_rate {
                    Some(dampen_rate) => node.dampen_rate = dampen_rate,
                    None => {}
                };
                match set_node_event.dampen_rate {
                    Some(dampen_rate) => node.dampen_rate = dampen_rate,
                    None => {}
                };
                match set_node_event.freeze {
                    Some(freeze) => node.freeze = freeze,
                    None => {}
                };
            }
            node::Event::Get(get_event) => get_event.handle(self),
            node::Event::Step(step_event) => {
                for _ in 0..step_event.steps {
                    self.step()
                }
            }
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

    #[test]
    fn updates_nodes_each_step() {
        let mut simulation = Simulation::new();
        let id = node::Id(1);
        let position_a = node::Position(cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let node_a = node::Node::new(id, position_a.clone());

        let id = node::Id(2);
        let position_b = node::Position(cgmath::Point3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        let node_b = node::Node::new(id, position_b.clone());

        simulation.add_node(node_a);
        simulation.add_node(node_b);

        simulation.step();

        let node_a = simulation
            .nodes
            .iter()
            .find(|node| node.id == node::Id(1))
            .unwrap();
        let node_b = simulation
            .nodes
            .iter()
            .find(|node| node.id == node::Id(2))
            .unwrap();

        assert_ne!(position_a, node_a.position);
        assert_ne!(position_b, node_b.position);
    }

    #[test]
    pub fn can_handle_step_event() {
        let mut simulation = Simulation::new();
        simulation.target_tps = 0;
        let node_a = node::Node {
            id: node::Id(1),
            position: node::Position::from((0.0, 0.0, 0.0)),
            velocity: node::Force::from((1.0, 0.0, 0.0)),
            mass: 1.0,
            gravitational_constant_override: None,
            dampen_rate: 0.0,
            freeze: false,
        };

        simulation.add_node(node_a);

        {
            let node = simulation
                .nodes
                .iter()
                .find(|node| node.id == node::Id(1))
                .unwrap();
            assert_eq!(node::Position::from((0.0, 0.0, 0.0)), node.position);
        }

        let step_event = node::Event::Step(node::event::step::StepEvent { steps: 1 });

        simulation.handle_event(step_event);

        {
            let node = simulation
                .nodes
                .iter()
                .find(|node| node.id == node::Id(1))
                .unwrap();
            assert_ne!(node::Position::from((0.0, 0.0, 0.0)), node.position)
        };
    }
}
