use crate::simulation;

use super::Scene;

pub struct ShimState {}

impl Scene for ShimState {
    fn new(_context: &sdl2::Sdl, _default_texture_path: Option<String>) -> Self
    where
        Self: Sized,
    {
        return ShimState {};
    }
    fn resize(&mut self, _new_size: (u32, u32)) {}
    fn input(&mut self, _event: &sdl2::event::Event) -> bool {
        false
    }
    fn update(&mut self) {}
    fn render(
        &mut self,
        _clear_colour: wgpu::Color,
        _simulation: &simulation::Simulation,
    ) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
