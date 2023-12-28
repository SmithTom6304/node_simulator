use node_simulator::node::{self};

#[derive(clap::Args, Debug)]
pub struct NodeArgs {
    #[arg(short, long)]
    id: u32,
    #[arg(short, long)]
    position: Option<String>,
}

impl TryFrom<&NodeArgs> for node::AddNodeEvent {
    type Error = NodeArgsError;

    fn try_from(value: &NodeArgs) -> Result<Self, Self::Error> {
        let id = node::Id(value.id);
        let position = match &value.position {
            Some(pos) => node::Position::try_from(pos.to_string()),
            None => Ok(node::Position::default()),
        };
        let position = match position {
            Ok(pos) => pos,
            Err(err) => return Err(NodeArgsError { message: err }),
        };
        let node = node::Node::new(id, position);
        Ok(node::AddNodeEvent { node })
    }
}

#[derive(Debug)]
pub struct NodeArgsError {
    pub message: String,
}

impl std::fmt::Display for NodeArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for NodeArgsError {}
