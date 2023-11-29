use crate::simulation;

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
    fn render(
        &mut self,
        clear_colour: wgpu::Color,
        simulation: &simulation::Simulation,
    ) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
