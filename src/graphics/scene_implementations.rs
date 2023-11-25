use crate::node;

pub mod state;

pub trait Scene {
    fn new(context: &sdl2::Sdl, default_texture_path: Option<String>) -> Self
    where
        Self: Sized;
    fn resize(&mut self, new_size: (u32, u32));
    fn input(&mut self, event: &sdl2::event::Event) -> bool;
    fn update(&mut self);
    fn add_node_to_scene(&mut self, node: node::Node);
    fn remove_node_from_scene(&mut self, id: node::NodeId);
    fn render(&mut self, clear_colour: wgpu::Color) -> Result<(), wgpu::SurfaceError>;
}
