use crate::simulation;

use super::node;
use winit::{event, event_loop, window};

mod camera;
mod instances;
mod models;
pub mod node_events;
pub mod state;
mod texture;
mod vertex;

pub struct GraphicsInterface<'a> {
    pub window: winit::window::Window,
    pub event_loop: winit::event_loop::EventLoop<node_events::NodeEvent>,
    pub simulation: &'a simulation::Simulation,
}

impl<'a> GraphicsInterface<'a> {
    pub fn new(simulation: &'a simulation::Simulation) -> GraphicsInterface<'a> {
        env_logger::init();
        let event_loop =
            event_loop::EventLoopBuilder::<node_events::NodeEvent>::with_user_event().build();
        let window = window::WindowBuilder::new().build(&event_loop).unwrap();

        GraphicsInterface {
            window,
            event_loop: event_loop,
            simulation,
        }
    }

    pub fn create_scene(self) {
        let window = self.window;
        let mut state = state::State::new(&window, None, self.simulation);

        self.event_loop
            .run(move |event, _, control_flow| match event {
                event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !state.input(event) {
                        match event {
                            event::WindowEvent::CloseRequested
                            | event::WindowEvent::KeyboardInput {
                                input:
                                    event::KeyboardInput {
                                        state: event::ElementState::Pressed,
                                        virtual_keycode: Some(event::VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = event_loop::ControlFlow::Exit,

                            event::WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                    state.update();
                    let clear_colour = wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    };
                    match state.render(clear_colour) {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = event_loop::ControlFlow::Exit
                        }
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                event::Event::UserEvent(event) => match event {
                    node_events::NodeEvent::Close => {
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    node_events::NodeEvent::Add(node) => state.add_node_to_scene(node),
                    node_events::NodeEvent::Remove(id) => state.remove_node_from_scene(id),
                },
                event::Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.request_redraw();
                }
                _ => {}
            });
    }
}
