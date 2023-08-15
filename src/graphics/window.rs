use winit::{event, event_loop};

pub struct Window {
    pub handle: winit::window::Window,
    event_loop: event_loop::EventLoop<()>,
}

impl Window {

    pub fn new() -> Self {
        let event_loop = event_loop::EventLoop::new();
        let handle = winit::window::Window::new(&event_loop).unwrap();
    
        Window {
            handle,
            event_loop
        }
    }

    fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.handle.inner_size()
    }

    fn set_size(&self, width: u32, height: u32) {
        let physical_size = winit::dpi::PhysicalSize::new(width, height);
        self.handle.set_inner_size(physical_size)
    }
}
