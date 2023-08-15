use crate::graphics::vertex;

pub trait Display {
   fn generate_list_of_vertices() -> Vec<vertex::Vertex>;
}
