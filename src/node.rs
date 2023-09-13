pub struct Node {
    pub x: i32,
    pub y: i32,
}

impl Node {
    pub fn new(x: i32, y: i32) -> Self {
        Node { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_created_from_x_and_y_coord() {
        let node = Node::new(5, 7);
        assert_eq!(5, node.x);
        assert_eq!(7, node.y);
    }
}
