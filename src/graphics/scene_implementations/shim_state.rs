use crate::node;

use super::Scene;

pub struct ShimState {}

impl Scene for ShimState {
    fn new(context: &sdl2::Sdl, default_texture_path: Option<String>) -> Self
    where
        Self: Sized,
    {
        return ShimState {};
    }
    fn resize(&mut self, new_size: (u32, u32)) {}
    fn input(&mut self, event: &sdl2::event::Event) -> bool {
        false
    }
    fn update(&mut self) {}
    fn add_node_to_scene(&mut self, node: node::Node) {}
    fn remove_node_from_scene(&mut self, id: node::NodeId) {}
    fn render(&mut self, clear_colour: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
