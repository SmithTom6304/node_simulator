use node_simulator::node;

#[derive(clap::Args, Debug)]
pub struct TpsArgs {
    pub target: u32,
}

impl From<&TpsArgs> for node::SetTargetTpsEvent {
    fn from(value: &TpsArgs) -> Self {
        Self {
            target_tps: value.target,
        }
    }
}
