use self::scene_implementations::Scene;
use crate::simulation;

use super::node;
use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::mpsc;
use std::time::Duration;

#[cfg(feature = "wgpu")]
mod camera;
#[cfg(feature = "wgpu")]
mod instances;
#[cfg(feature = "wgpu")]
mod models;
pub mod node_events;
mod scene_implementations;
#[cfg(feature = "wgpu")]
mod texture;
#[cfg(feature = "wgpu")]
mod vertex;

pub struct GraphicsInterface<'a> {
    pub simulation: &'a mut simulation::Simulation,
    pub context: sdl2::Sdl,
    pub event: sdl2::EventSubsystem,
    pub scene: Box<dyn scene_implementations::Scene>,
}

enum EventStatus {
    Handled,
    Close,
}

impl<'a> GraphicsInterface<'a> {
    pub fn new(
        simulation: &'a mut simulation::Simulation,
        create_display: bool,
    ) -> GraphicsInterface<'a> {
        let context = sdl2::init().unwrap();
        let event = context.event().unwrap();
        match event.register_custom_event::<node_events::NodeEvent>() {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        };
        let scene = Self::init_scene(&context, create_display);

        GraphicsInterface {
            simulation,
            context,
            event,
            scene,
        }
    }

    #[cfg(feature = "wgpu")]
    fn init_scene(
        context: &sdl2::Sdl,
        create_display: bool,
    ) -> Box<dyn scene_implementations::Scene> {
        match create_display {
            true => Box::new(scene_implementations::state::State::new(context, None)),
            false => Box::new(scene_implementations::shim_state::ShimState::new(
                context, None,
            )),
        }
    }

    #[cfg(not(feature = "wgpu"))]
    fn init_scene(
        context: &sdl2::Sdl,
        create_display: bool,
    ) -> Box<dyn scene_implementations::Scene> {
        Box::new(scene_implementations::shim_state::ShimState::new(
            context, None,
        ))
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

            let result = self.scene.render(
                wgpu::Color {
                    r: 0.65,
                    g: 0.68,
                    b: 0.97,
                    a: 1.0,
                },
                self.simulation,
            );
            if result.is_err() {
                println!("Render error - {}", result.err().unwrap());
            }
            self.scene.update();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    #[cfg(feature = "wgpu")]
    pub fn toggle_state(&mut self) {
        self.scene = match self
            .scene
            .as_any()
            .downcast_ref::<scene_implementations::state::State>()
        {
            Some(_) => Box::new(scene_implementations::shim_state::ShimState::new(
                &self.context,
                None,
            )),
            None => Box::new(scene_implementations::state::State::new(
                &self.context,
                None,
            )),
        };
    }

    #[cfg(not(feature = "wgpu"))]
    pub fn toggle_state(&mut self) {
        println!("Feature 'wgpu' is required to display scene.");
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
                self.simulation.add_node(add.node);
                return EventStatus::Handled;
            }
            None => (),
        };
        match event.remove_node_event {
            Some(remove) => {
                self.simulation.remove_node(remove.node_id);
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
        match event.toggle_scene_event {
            Some(_) => {
                self.toggle_state();
                return EventStatus::Handled;
            }
            None => (),
        };
        EventStatus::Handled
    }
}

#[cfg(test)]
mod a_graphics_interface {
    use std::thread;

    use super::{node_events::CloseEvent, *};
    use crate::simulation::Simulation;

    #[test]
    fn runs_until_a_close_event_is_received() {
        let mut simulation = Simulation::new();
        let graphics_interface = GraphicsInterface::new(&mut simulation, false);
        let (tx, rx) = mpsc::channel::<node_events::NodeEvent>();
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            send_close_event(&tx);
        });
        graphics_interface.run(rx);
    }

    #[test]
    fn can_receive_add_node_event() {
        let mut simulation = Simulation::new();
        let graphics_interface = GraphicsInterface::new(&mut simulation, false);
        let (tx, rx) = mpsc::channel::<node_events::NodeEvent>();
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            let id = node::Id(1);
            let position = Default::default();
            let node = node::Node::new(id, position);
            let add_node_event = node_events::NodeEvent {
                add_node_event: Some(node_events::AddNodeEvent { node }),
                ..Default::default()
            };
            send_event(&tx, add_node_event);
            thread::sleep(Duration::new(1, 0));
            send_close_event(&tx);
        });
        graphics_interface.run(rx);

        assert!(simulation.nodes.iter().any(|node| node.id().0 == 1));
    }

    #[test]
    fn can_receive_remove_node_event() {
        let mut simulation = Simulation::new();
        let node_id = node::Id(1);
        let id = node_id.clone();
        let position = Default::default();
        let node = node::Node::new(id, position);
        simulation.add_node(node);
        let graphics_interface = GraphicsInterface::new(&mut simulation, false);
        let (tx, rx) = mpsc::channel::<node_events::NodeEvent>();
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            let remove_node_event = node_events::NodeEvent {
                remove_node_event: Some(node_events::RemoveNodeEvent { node_id }),
                ..Default::default()
            };
            send_event(&tx, remove_node_event);
            thread::sleep(Duration::new(1, 0));
            send_close_event(&tx);
        });
        graphics_interface.run(rx);

        assert!(simulation.nodes.is_empty());
    }

    fn send_close_event(tx: &mpsc::Sender<node_events::NodeEvent>) {
        let close_event = node_events::NodeEvent {
            close_node_event: Some(CloseEvent {}),
            ..Default::default()
        };
        send_event(tx, close_event);
    }

    fn send_event(tx: &mpsc::Sender<node_events::NodeEvent>, event: node_events::NodeEvent) {
        _ = tx.send(event);
    }
}
