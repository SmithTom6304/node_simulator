use super::node;

pub enum NodeEvent {
    Add(node::Node),
    Remove(node::NodeId),
    Close,
}
