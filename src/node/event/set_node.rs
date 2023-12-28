use crate::node;

pub struct SetNodeEvent {
    pub id: node::Id,
    pub position: Option<node::Position>,
    pub velocity: Option<node::Force>,
    pub mass: Option<f32>,
    pub gravitational_constant_override: Option<f32>,
    pub dampen_rate: Option<f32>,
    pub freeze: Option<bool>,
}
