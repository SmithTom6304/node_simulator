pub mod vertex;
mod window;

pub struct Graphics {
    window: window::Window,
}

impl Graphics {
    pub fn new() -> Self {
        let window = window::Window::new();
        Graphics {
            window,
        }
    }
}

