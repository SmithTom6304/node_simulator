use crate::node::Node;

use super::Event;

pub struct AddNodeEvent {
    pub node: Node,
}

impl AddNodeEvent {
    pub fn new(node: Node) -> Event {
        Event {
            add_node_event: Some(AddNodeEvent { node }),
            ..Default::default()
        }
    }
}

impl From<AddNodeEvent> for Event {
    fn from(value: AddNodeEvent) -> Self {
        Event {
            add_node_event: Some(value),
            ..Default::default()
        }
    }
}
