#[derive(clap::Args, Debug)]
pub struct NodeArgs {
    #[arg(long)]
    pub id: u32,
    #[arg(long)]
    pub position: bool,
    #[arg(long)]
    pub velocity: bool,
    #[arg(long)]
    pub mass: bool,
    #[arg(long)]
    pub gravitational_constant_override: bool,
    #[arg(long)]
    pub dampen_rate: bool,
    #[arg(long)]
    pub freeze: bool,
}

impl From<NodeArgs> for node_simulator::node::event::get::NodeArgs {
    fn from(value: NodeArgs) -> Self {
        Self {
            id: value.id,
            position: value.position,
            velocity: value.velocity,
            mass: value.mass,
            gravitational_constant_override: value.gravitational_constant_override,
            dampen_rate: value.dampen_rate,
            freeze: value.freeze,
        }
    }
}

impl From<&NodeArgs> for node_simulator::node::event::get::NodeArgs {
    fn from(value: &NodeArgs) -> Self {
        Self {
            id: value.id,
            position: value.position,
            velocity: value.velocity,
            mass: value.mass,
            gravitational_constant_override: value.gravitational_constant_override,
            dampen_rate: value.dampen_rate,
            freeze: value.freeze,
        }
    }
}
