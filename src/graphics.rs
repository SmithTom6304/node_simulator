use self::scene_implementations::Scene;
use crate::simulation::{self, Simulation};

use super::node;
use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{self, Duration};

#[cfg(feature = "wgpu")]
mod camera;
#[cfg(feature = "wgpu")]
mod instances;
#[cfg(feature = "wgpu")]
mod models;
pub mod scene_event;
mod scene_implementations;
#[cfg(feature = "wgpu")]
mod texture;
#[cfg(feature = "wgpu")]
mod vertex;

pub struct GraphicsInterface {
    pub simulation_rx: mpsc::Receiver<Arc<Mutex<simulation::Simulation>>>,
    pub context: sdl2::Sdl,
    pub event: sdl2::EventSubsystem,
    pub scene: Box<dyn scene_implementations::Scene>,
}

enum EventStatus {
    Handled,
    Close,
}

impl GraphicsInterface {
    pub fn new(
        simulation_rx: mpsc::Receiver<Arc<Mutex<simulation::Simulation>>>,
        create_display: bool,
    ) -> GraphicsInterface {
        let context = sdl2::init().unwrap();
        let event = context.event().unwrap();
        match event.register_custom_event::<scene_event::SceneEvent>() {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        };
        let scene = Self::init_scene(&context, create_display);

        GraphicsInterface {
            simulation_rx,
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

    pub fn run(mut self, rx: mpsc::Receiver<scene_event::SceneEvent>) {
        let mut event_pump = self.context.event_pump().unwrap();
        let target_fps = 60;
        let target_duration = Duration::new(1, 0) / target_fps;
        let print_poor_performance = false;
        let mut simulation = self.try_update_simulation();
        'running: loop {
            let start_time = time::Instant::now();
            simulation = match self.try_update_simulation() {
                Some(sim) => Some(sim),
                None => simulation,
            };
            let ev = rx.try_recv();
            match ev {
                Ok(e) => _ = self.event.push_custom_event(e),
                Err(_) => (),
            }

            for event in event_pump.poll_iter() {
                let status = match event.is_user_event() {
                    true => self.handle_custom_event(
                        event
                            .as_user_event_type::<scene_event::SceneEvent>()
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

            let mut render_scene = |simulation: Option<&Simulation>| {
                let result = self.scene.render(
                    wgpu::Color {
                        r: 0.65,
                        g: 0.68,
                        b: 0.97,
                        a: 1.0,
                    },
                    simulation,
                );

                if result.is_err() {
                    println!("Render error - {}", result.err().unwrap());
                }
            };

            render_scene(simulation.as_ref());

            self.scene.update();

            let duration = time::Instant::now().duration_since(start_time);
            match duration.cmp(&target_duration) {
                std::cmp::Ordering::Less => std::thread::sleep(target_duration - duration),
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => match print_poor_performance {
                    true => println!("Poor performance - target frame duration was {:?}, achieved frame duration was {:?}", target_duration, duration),
                    false => {},
                },
            }
        }
    }

    fn try_update_simulation(&self) -> Option<Simulation> {
        match self.simulation_rx.try_recv() {
            Ok(sim) => match sim.try_lock() {
                Ok(sim) => Some(sim.clone()),
                Err(_) => None,
            },
            Err(_) => None,
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

    fn handle_custom_event(&mut self, event: scene_event::SceneEvent) -> EventStatus {
        match event.close_event {
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

    use super::{scene_event::CloseEvent, *};
    use crate::simulation::Simulation;

    #[test]
    fn runs_until_a_close_event_is_received() {
        let mut simulation = Simulation::new();
        let (simulation_tx, simulation_rx) = mpsc::channel();
        let (node_event_tx, node_event_rx) = mpsc::channel::<scene_event::SceneEvent>();
        let graphics_interface = GraphicsInterface::new(simulation_rx, false);
        let (tx, rx) = mpsc::channel::<scene_event::SceneEvent>();
        thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            send_close_event(&tx);
        });
        graphics_interface.run(rx);
    }

    // #[test]
    // fn can_receive_add_node_event() {
    //     let mut simulation = Simulation::new();
    //     let (simulation_tx, simulation_rx) = mpsc::channel();
    //     let (node_event_tx, node_event_rx) = mpsc::channel::<scene_event::SceneEvent>();
    //     let graphics_interface = GraphicsInterface::new(simulation_rx, false);
    //     let (tx, rx) = mpsc::channel::<scene_event::SceneEvent>();
    //     thread::spawn(move || {
    //         thread::sleep(Duration::new(1, 0));
    //         let id = node::Id(1);
    //         let position = Default::default();
    //         let node = node::Node::new(id, position);
    //         let add_node_event = scene_event::SceneEvent {
    //             add_node_event: Some(scene_event::AddNodeEvent { node }),
    //             ..Default::default()
    //         };
    //         send_event(&tx, add_node_event);
    //         thread::sleep(Duration::new(1, 0));
    //         send_close_event(&tx);
    //     });
    //     graphics_interface.run(rx);

    //     assert!(simulation.nodes.iter().any(|node| node.id().0 == 1));
    // }

    // #[test]
    // fn can_receive_remove_node_event() {
    //     let mut simulation = Simulation::new();
    //     let node_id = node::Id(1);
    //     let id = node_id.clone();
    //     let position = Default::default();
    //     let node = node::Node::new(id, position);
    //     simulation.add_node(node);
    //     let (simulation_tx, simulation_rx) = mpsc::channel();
    //     let (node_event_tx, node_event_rx) = mpsc::channel::<scene_event::SceneEvent>();
    //     let graphics_interface = GraphicsInterface::new(node_event_tx, simulation_rx, false);
    //     let (tx, rx) = mpsc::channel::<scene_event::SceneEvent>();
    //     thread::spawn(move || {
    //         thread::sleep(Duration::new(1, 0));
    //         let remove_node_event = scene_event::SceneEvent {
    //             remove_node_event: Some(scene_event::RemoveNodeEvent { node_id }),
    //             ..Default::default()
    //         };
    //         send_event(&tx, remove_node_event);
    //         thread::sleep(Duration::new(1, 0));
    //         send_close_event(&tx);
    //     });
    //     graphics_interface.run(rx);

    //     assert!(simulation.nodes.is_empty());
    // }

    fn send_close_event(tx: &mpsc::Sender<scene_event::SceneEvent>) {
        let close_event = scene_event::SceneEvent {
            close_event: Some(CloseEvent {}),
            ..Default::default()
        };
        send_event(tx, close_event);
    }

    fn send_event(tx: &mpsc::Sender<scene_event::SceneEvent>, event: scene_event::SceneEvent) {
        _ = tx.send(event);
    }
}
