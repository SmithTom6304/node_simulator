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
            Some(pos) => NodeArgs::try_from_string(pos.to_string()),
            None => Ok(node::Position::default()),
        };
        let position = match position {
            Ok(pos) => pos,
            Err(err) => return Err(err),
        };
        let node = node::Node::new(id, position);
        Ok(node::AddNodeEvent { node })
    }
}

impl NodeArgs {
    fn try_from_string(value: String) -> Result<node::Position, NodeArgsError> {
        let pos_string = value.trim_matches('"').split(',');
        let positions: Vec<Result<f32, _>> = pos_string.map(|s| s.parse::<f32>()).collect();
        if positions.len() != 3 {
            return Err(NodeArgsError {
                message: "Position must have 3 values".to_string(),
            });
        }
        let x = match &positions[0] {
            Ok(number) => *number,
            Err(_) => {
                return Err(NodeArgsError {
                    message: "Position x must be an f32".to_string(),
                });
            }
        };
        let y = match &positions[1] {
            Ok(number) => *number,
            Err(_) => {
                return Err(NodeArgsError {
                    message: "Position y must be an f32".to_string(),
                });
            }
        };
        let z = match &positions[2] {
            Ok(number) => *number,
            Err(_) => {
                return Err(NodeArgsError {
                    message: "Position z must be an f32".to_string(),
                });
            }
        };
        Ok(node::Position(cgmath::Point3 { x, y, z }))
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
