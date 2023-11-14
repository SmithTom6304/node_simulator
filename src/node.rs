use std::fmt;

pub struct Node {
    pub position: NodePosition,
}

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
    pub fn new(position: NodePosition) -> Self {
        Node { position }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_created_from_node_position() {
        let node = Node::new(NodePosition { x: 5, y: 7 });
        assert_eq!(5, node.position.x);
        assert_eq!(7, node.position.y);
    }
}
