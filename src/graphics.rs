use crate::simulation;

use self::scene_implementations::Scene;

use super::node;
use sdl2;

use scene_implementations::state;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::mpsc;
use std::time::Duration;

mod camera;
mod instances;
mod models;
pub mod node_events;
mod scene_implementations;
mod texture;
mod vertex;

pub struct GraphicsInterface<'a> {
    pub simulation: &'a simulation::Simulation,
    pub context: sdl2::Sdl,
    pub event: sdl2::EventSubsystem,
    pub scene: Box<dyn scene_implementations::Scene>,
}

enum EventStatus {
    Handled,
    Close,
}

impl<'a> GraphicsInterface<'a> {
    pub fn new(simulation: &'a simulation::Simulation) -> GraphicsInterface<'a> {
        let context = sdl2::init().unwrap();
        let event = context.event().unwrap();
        let scene: Box<dyn scene_implementations::Scene> =
            Box::new(state::State::new(&context, None));

        GraphicsInterface {
            simulation,
            context,
            event,
            scene,
        }
    }

    pub fn run(mut self, rx: mpsc::Receiver<node_events::NodeEvent>) {
        let mut event_pump = self.context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;

            let ev = rx.try_recv();
            match ev {
                Ok(e) => _ = self.event.push_custom_event(e),
                Err(_) => (),
            }

            for event in event_pump.poll_iter() {
                let status = match event.is_user_event() {
                    true => self.handle_custom_event(
                        event
                            .as_user_event_type::<node_events::NodeEvent>()
                            .expect("User event was not node event"),
                    ),
                    false => self.handle_event(event),
                };
                match status {
                    EventStatus::Close => break 'running,
                    EventStatus::Handled => (),
                }
            }
            // The rest of the game loop goes here...

            let result = self.scene.render(wgpu::Color {
                r: 0.65,
                g: 0.68,
                b: 0.97,
                a: 1.0,
            });
            if result.is_err() {
                println!("Render error - {}", result.err().unwrap());
            }
            self.scene.update();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn handle_event(&mut self, event: sdl2::event::Event) -> EventStatus {
        if self.scene.input(&event) {
            return EventStatus::Handled;
        }
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => EventStatus::Close,
            Event::Window { win_event, .. } => match win_event {
                sdl2::event::WindowEvent::Resized(w, h) => {
                    self.scene.resize((w as u32, h as u32));
                    EventStatus::Handled
                }
                _ => EventStatus::Handled,
            },
            _ => EventStatus::Handled,
        }
    }

    fn handle_custom_event(&mut self, event: node_events::NodeEvent) -> EventStatus {
        match event.add_node_event {
            Some(add) => {
                self.scene.add_node_to_scene(add.node);
                return EventStatus::Handled;
            }
            None => (),
        };
        match event.remove_node_event {
            Some(remove) => {
                self.scene.remove_node_from_scene(remove.node_id);
                return EventStatus::Handled;
            }
            None => (),
        };
        match event.close_node_event {
            Some(_) => {
                return EventStatus::Close;
            }
            None => (),
        };
        EventStatus::Handled
    }
}
