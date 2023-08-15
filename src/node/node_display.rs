use crate::{display, graphics::vertex};

struct NodeDisplay {
    node: super::Node,
    colour: (u8, u8, u8),
    size: u32,
    sample_count: u32,
}

impl display::Display for NodeDisplay {
    fn generate_list_of_vertices() -> Vec<vertex::Vertex> {
        todo!()
        // Generate vertices of a circle with sample_count
    }
}
