use std::fmt;

pub mod event;
mod force;
pub mod id;
pub mod position;

pub use event::{AddNodeEvent, Event, RemoveNodeEvent, SetTargetTpsEvent};
pub use id::Id;
pub use position::Position;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Node {
    id: Id,
    position: Position,
    pub incoming_force: force::Force,
    pub outgoing_force: force::Force,
    //TODO Added for debugging purposes
    toggle: bool,
}

impl Node {
    pub fn new(id: Id, position: Position) -> Self {
        Node {
            id,
            position,
            incoming_force: force::Force(cgmath::Vector3::<f32>::new(0.0, 0.0, 0.0)),
            outgoing_force: force::Force(cgmath::Vector3::<f32>::new(0.0, 0.0, 0.0)),
            toggle: true,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position
    }

    fn update_position(&mut self) {
        self.position.0 -= self.incoming_force.0;
        // Dampen
        self.incoming_force.0 *= 0.9;
    }

    pub fn step<F>(&mut self, mut node_force_function: F) -> ()
    where
        F: FnMut(&mut Self) -> cgmath::Vector3<f32>,
    {
        let internal_force = node_force_function(self);
        self.incoming_force.0 += internal_force;
        self.update_position();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_created_from_id_and_position() {
        let node = Node::new(
            Id(3),
            Position(cgmath::Point3 {
                x: 5.0,
                y: 7.0,
                z: 9.0,
            }),
        );
        assert_eq!(5.0, node.position.0.x);
        assert_eq!(7.0, node.position.0.y);
        assert_eq!(9.0, node.position.0.z);
    }
}
