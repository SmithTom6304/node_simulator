use super::node;

pub enum NodeEvent {
    Add(node::Node),
    Close,
}
