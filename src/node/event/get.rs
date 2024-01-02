use crate::{
    node::{self, Node},
    simulation::Simulation,
};

pub enum GetEvent {
    Node(NodeArgs),
    Tps,
}

pub struct NodeArgs {
    pub id: u32,
    pub position: bool,
    pub velocity: bool,
    pub mass: bool,
    pub gravitational_constant_override: bool,
    pub dampen_rate: bool,
    pub freeze: bool,
}

impl GetEvent {
    pub fn handle(&self, simulation: &Simulation) {
        match self {
            GetEvent::Node(node_args) => node_args.display_node_information(simulation),
            GetEvent::Tps => println!("tps: {}", simulation.target_tps()),
        }
    }
}

impl NodeArgs {
    pub fn display_node_information(&self, simulation: &Simulation) {
        let node = match simulation
            .nodes
            .iter()
            .find(|node| node.id == node::Id(self.id))
        {
            Some(node) => node,
            None => {
                println!("Error displaying node information for node with id {} - no node with that id exists", self.id);
                return;
            }
        };

        println!("{}", self.get_display_string_from_node_args(node));
    }

    fn get_display_string_from_node_args(&self, node: &Node) -> String {
        let mut display_string = format!("Node {}:", node.id.to_string());
        // TODO - Use bitflags crate - https://docs.rs/bitflags/latest/bitflags/
        let no_flags_present = !(self.position
            || self.velocity
            || self.mass
            || self.gravitational_constant_override
            || self.dampen_rate
            || self.freeze);

        display_string = match self.position || no_flags_present {
            true => format!(
                "{display_string}\n\tposition: {}",
                node.position.to_string()
            ),
            false => display_string,
        };

        display_string = match self.velocity || no_flags_present {
            true => format!(
                "{display_string}\n\tvelocity: {}",
                node.velocity.to_string()
            ),
            false => display_string,
        };

        display_string = match self.mass || no_flags_present {
            true => format!("{display_string}\n\tmass: {}", node.mass.to_string()),
            false => display_string,
        };

        display_string = match self.gravitational_constant_override || no_flags_present {
            true => {
                let value = match node.gravitational_constant_override {
                    Some(value) => value.to_string(),
                    None => "None".to_string(),
                };
                format!("{display_string}\n\tgravitational constant override: {value}")
            }
            false => display_string,
        };

        display_string = match self.dampen_rate || no_flags_present {
            true => format!(
                "{display_string}\n\tdampen rate: {}",
                node.dampen_rate.to_string()
            ),
            false => display_string,
        };

        display_string = match self.freeze || no_flags_present {
            true => format!("{display_string}\n\tfreeze: {}", node.freeze.to_string()),
            false => display_string,
        };

        display_string
    }
}
