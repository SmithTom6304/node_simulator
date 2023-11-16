use std::fmt;

pub struct Node {
    pub id: NodeId,
    pub position: NodePosition,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct NodeId(pub u32);

pub struct NodePosition {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for NodePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl Node {
    pub fn new(id: NodeId, position: NodePosition) -> Self {
        Node { id, position }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_created_from_id_and_position() {
        let node = Node::new(NodeId(3), NodePosition { x: 5, y: 7 });
        assert_eq!(5, node.position.x);
        assert_eq!(7, node.position.y);
    }
}
