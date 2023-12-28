use node_simulator::node::{self, event::set_node};

#[derive(clap::Args, Debug)]
pub struct NodeArgs {
    #[arg(long)]
    id: u32,
    #[arg(long)]
    position: Option<String>,
    #[arg(long)]
    velocity: Option<String>,
    #[arg(long)]
    mass: Option<f32>,
    #[arg(long)]
    gravitational_constant_override: Option<f32>,
    #[arg(long)]
    dampen_rate: Option<f32>,
    #[arg(long)]
    freeze: Option<bool>,
}

impl TryFrom<&NodeArgs> for set_node::SetNodeEvent {
    type Error = String;

    fn try_from(value: &NodeArgs) -> Result<Self, Self::Error> {
        let id = node::Id(value.id);
        let position = match &value.position {
            Some(position) => match node::Position::try_from(position.clone()) {
                Ok(position) => Some(position),
                Err(err) => return Err(err),
            },
            None => None,
        };
        let velocity = match &value.velocity {
            Some(velocity) => match node::Force::try_from(velocity.clone()) {
                Ok(velocity) => Some(velocity),
                Err(err) => return Err(err),
            },
            None => None,
        };
        let mass = value.mass;
        let gravitational_constant_override = value.gravitational_constant_override;
        let dampen_rate = value.dampen_rate;
        let freeze = value.freeze;

        Ok(Self {
            id,
            position,
            velocity,
            mass,
            gravitational_constant_override,
            dampen_rate,
            freeze,
        })
    }
}
