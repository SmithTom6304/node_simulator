use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Node {
    id: NodeId,
    position: NodePosition,
}

impl Node {
    pub fn new(id: NodeId, position: NodePosition) -> Self {
        Node { id, position }
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn position(&self) -> &NodePosition {
        &self.position
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct NodeId(pub u32);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct NodePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl fmt::Display for NodePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_created_from_id_and_position() {
        let node = Node::new(NodeId(3), NodePosition { x: 5, y: 7, z: 9 });
        assert_eq!(5, node.position.x);
        assert_eq!(7, node.position.y);
        assert_eq!(9, node.position.z);
    }
}
