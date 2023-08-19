use winit::{event_loop::{self, EventLoopBuilder}, window, event};

use self::state::State;

pub mod vertex;
mod state;
mod camera;
mod texture;

pub fn init() {
    env_logger::init();
}

pub fn get_event_loop() -> event_loop::EventLoop<event::WindowEvent<'static>> {
    let mut event_loop_builder: EventLoopBuilder<event::WindowEvent> = event_loop::EventLoopBuilder::with_user_event();
    event_loop_builder.build()
}

pub async fn run(event_loop: event_loop::EventLoop<event::WindowEvent<'static>>) {
    
    let window = window::WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        event::Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => {
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
        event::Event::RedrawRequested(window_id) if window_id == state.window().id() => {
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
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = event_loop::ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        event::Event::UserEvent(event) => {
            match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                },
                _ => {}
            }
        }
        event::Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            state.window().request_redraw();
        }
        _ => {}
    });
}

