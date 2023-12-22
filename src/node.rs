pub mod event;
pub mod force;
pub mod id;
pub mod position;

pub use event::{AddNodeEvent, Event, RemoveNodeEvent, SetTargetTpsEvent};
pub use force::Force;
pub use id::Id;
pub use position::Position;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Node {
    pub id: Id,
    pub position: Position,
    pub velocity: force::Force,
    pub mass: f32,
    pub gravitational_constant_override: Option<f32>,
    /// Rate at which to dampen a nodes velocity. 0 is no dampening, 1 is instant dampening.
    pub dampen_rate: f32,
}

impl Node {
    pub fn new(id: Id, position: Position) -> Self {
        Node {
            id,
            position,
            velocity: force::Force(cgmath::Vector3::<f32>::new(0.0, 0.0, 0.0)),
            mass: 1.0,
            gravitational_constant_override: None,
            dampen_rate: 0.1,
        }
    }

    fn update_position(&mut self) {
        self.position = self.position + self.velocity;
        // Dampen
        self.velocity.0 *= 1.0 - self.dampen_rate;
    }

    pub fn step<F>(&mut self, mut node_force_function: F) -> ()
    where
        F: FnMut(&mut Self) -> Force,
    {
        let internal_force = node_force_function(self);
        self.velocity += internal_force;
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
        let (x, y, z) = node.position.into();
        assert_eq!(5.0, x);
        assert_eq!(7.0, y);
        assert_eq!(9.0, z);
    }
}
