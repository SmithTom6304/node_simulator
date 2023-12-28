use crate::node::Id;

use super::Event;

pub struct RemoveNodeEvent {
    pub node_id: Id,
}

impl RemoveNodeEvent {
    pub fn new(id: Id) -> Event {
        Event {
            remove_node_event: Some(RemoveNodeEvent { node_id: id }),
            ..Default::default()
        }
    }
}

impl From<RemoveNodeEvent> for Event {
    fn from(value: RemoveNodeEvent) -> Self {
        Event {
            remove_node_event: Some(value),
            ..Default::default()
        }
    }
}
