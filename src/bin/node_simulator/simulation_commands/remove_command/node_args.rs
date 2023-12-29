use node_simulator::node;

#[derive(clap::Args, Debug)]
pub struct NodeArgs {
    #[arg(short, long)]
    id: u32,
}

impl From<&NodeArgs> for node::RemoveNodeEvent {
    fn from(value: &NodeArgs) -> Self {
        let id = node::Id(value.id);
        node::RemoveNodeEvent { node_id: id }
    }
}
