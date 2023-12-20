use std::fmt;

pub mod event;
pub mod id;
pub mod position;

pub use event::{AddNodeEvent, Event, RemoveNodeEvent, SetTargetTpsEvent};
pub use id::Id;
pub use position::Position;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Node {
    id: Id,
    position: Position,
    //TODO Added for debugging purposes
    toggle: bool,
}

impl Node {
    pub fn new(id: Id, position: Position) -> Self {
        Node {
            id,
            position,
            toggle: true,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn step(&mut self) {
        self.toggle = !self.toggle;
        match self.toggle {
            true => self.position.x += 1,
            false => self.position.x -= 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_created_from_id_and_position() {
        let node = Node::new(Id(3), Position { x: 5, y: 7, z: 9 });
        assert_eq!(5, node.position.x);
        assert_eq!(7, node.position.y);
        assert_eq!(9, node.position.z);
    }
}
