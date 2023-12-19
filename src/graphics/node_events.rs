use super::node;

#[derive(Default)]
pub struct NodeEvent {
    pub add_node_event: Option<AddNodeEvent>,
    pub remove_node_event: Option<RemoveNodeEvent>,
    pub close_node_event: Option<CloseEvent>,
    pub toggle_scene_event: Option<ToggleSceneEvent>,
}

pub struct AddNodeEvent {
    pub node: node::Node,
}

impl AddNodeEvent {
    pub fn new(node: node::Node) -> NodeEvent {
        NodeEvent {
            add_node_event: Some(AddNodeEvent { node }),
            ..Default::default()
        }
    }
}

pub struct RemoveNodeEvent {
    pub node_id: node::Id,
}

impl RemoveNodeEvent {
    pub fn new(id: node::Id) -> NodeEvent {
        NodeEvent {
            remove_node_event: Some(RemoveNodeEvent { node_id: id }),
            ..Default::default()
        }
    }
}

pub struct CloseEvent {}

impl CloseEvent {
    pub fn new() -> NodeEvent {
        NodeEvent {
            close_node_event: Some(CloseEvent {}),
            ..Default::default()
        }
    }
}

pub struct ToggleSceneEvent {}
impl ToggleSceneEvent {
    pub fn new() -> NodeEvent {
        NodeEvent {
            toggle_scene_event: Some(ToggleSceneEvent {}),
            ..Default::default()
        }
    }
}
