pub mod event;
pub mod force;
pub mod id;
pub mod position;

pub use event::{
    add_node::AddNodeEvent, remove_node::RemoveNodeEvent, set_target_tps::SetTargetTpsEvent, Event,
};
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
    /// Stops the node from moving when undergoing a force. This node will still exert force on other
    /// nodes - this can be disabled via the gravitational_constant_override
    pub freeze: bool,
}

impl Node {
    const MIN_VELOCITY: f32 = 0.0001;

    pub fn new(id: Id, position: Position) -> Self {
        Node {
            id,
            position,
            velocity: force::Force(cgmath::Vector3::<f32>::new(0.0, 0.0, 0.0)),
            mass: 1.0,
            gravitational_constant_override: None,
            dampen_rate: 0.1,
            freeze: false,
        }
    }

    fn update_position(&mut self) {
        self.position = match self.freeze {
            true => self.position,
            false => self.position + self.velocity,
        };
        // Dampen
        self.velocity = self.velocity * (1.0 - self.dampen_rate);
        let velocity_magnitude = self.velocity.magnitude();
        if 0.0 < velocity_magnitude && velocity_magnitude < Self::MIN_VELOCITY {
            self.velocity = Force::zero();
        }
    }

    pub fn step<F>(&mut self, mut node_force_function: F) -> ()
    where
        F: FnMut(&mut Self) -> Force,
    {
        let internal_force = node_force_function(self);
        self.velocity += internal_force * (1.0 / self.mass);
        self.update_position();
    }
}

#[cfg(test)]
mod a_node {
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

    #[test]
    fn updates_its_velocity_every_frame_while_undergoing_a_force() {
        let mut node = Node::new(Id(1), Position::default());
        node.dampen_rate = 0.0; // Easier to compare with no dampening
        let expected_velocity = Force(cgmath::Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        let node_force_function = |mut _node: &mut Node| expected_velocity.clone();

        node.step(node_force_function);

        assert_eq!(expected_velocity, node.velocity);

        node.step(node_force_function);

        assert_eq!(expected_velocity * 2.0, node.velocity);
    }

    #[test]
    fn dampens_velocity_based_on_dampen_rate() {
        let mut node = Node::new(Id(1), Position::default());
        node.dampen_rate = 0.5;
        let expected_velocity = Force(cgmath::Vector3 {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        });
        let node_force_function = |mut _node: &mut Node| {
            Force(cgmath::Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            })
        };
        node.step(node_force_function);
        assert_eq!(expected_velocity, node.velocity);

        // Stop applying force to node
        let node_force_function = |mut _node: &mut Node| Force::zero();
        node.step(node_force_function);
        assert_eq!(expected_velocity * 0.5, node.velocity);
    }

    #[test]
    fn does_not_move_if_frozen() {
        let mut node = Node::new(Id(1), Position::default());
        node.freeze = true;
        let expected_position = Position::default();
        let node_force_function = |mut _node: &mut Node| {
            Force(cgmath::Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            })
        };
        node.step(node_force_function);
        assert!(
            node.velocity.magnitude() > 0.0,
            "Node velocity is still updated..."
        );
        assert!(
            expected_position == node.position,
            "...but its position is not!"
        );
    }
}
