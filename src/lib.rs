use std::io;
use std::thread;

use graphics::node_events;
use graphics::state;
use node::NodePosition;
use winit::event_loop;

mod graphics;
mod node;
mod node_collection;
mod resources;

const QUIT_COMMAND: &str = ":q";
const ADD_NODE_COMMAND: &str = ":a";

pub fn run(default_texture_path: Option<String>) {
    graphics::init();
    let event_loop = graphics::get_event_loop();
    let event_loop_proxy = event_loop.create_proxy();
    let window = graphics::create_window(&event_loop);
    let mut state = pollster::block_on(state::State::new(window, default_texture_path));

    thread::spawn(|| {
        println!("Running node_simulator...");
        help();
        read_input(event_loop_proxy);
    });
    pollster::block_on(graphics::run(event_loop, state));
}

fn help() {
    println!("Enter commands via the CLI.");
    println!("Enter :h for help, :q to quit, :a to add node");
}

fn read_input(event_loop_proxy: event_loop::EventLoopProxy<node_events::NodeEvent>) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        match input {
            QUIT_COMMAND => {
                event_loop_proxy
                    .send_event(node_events::NodeEvent::Close)
                    .ok();
                break;
            }
            ADD_NODE_COMMAND => {
                let node = node::Node::new(NodePosition { x: 5, y: 7 });
                event_loop_proxy
                    .send_event(node_events::NodeEvent::Add(node))
                    .ok();
            }
            ":h" => help(),
            _ => (),
        }
    }
}
