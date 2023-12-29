use super::{Id, Node};

#[derive(Default)]
pub struct Event {
    pub add_node_event: Option<AddNodeEvent>,
    pub remove_node_event: Option<RemoveNodeEvent>,
    pub set_target_tps_event: Option<SetTargetTpsEvent>,
}

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

pub struct SetTargetTpsEvent {
    pub target_tps: Option<u32>,
}

impl SetTargetTpsEvent {
    pub fn new(target_tps: Option<u32>) -> Event {
        Event {
            set_target_tps_event: Some(SetTargetTpsEvent { target_tps }),
            ..Default::default()
        }
    }
}

impl From<SetTargetTpsEvent> for Event {
    fn from(value: SetTargetTpsEvent) -> Self {
        Event {
            set_target_tps_event: Some(value),
            ..Default::default()
        }
    }
}
