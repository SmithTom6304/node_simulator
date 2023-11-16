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
        let mut input = input.trim().split_ascii_whitespace().peekable();

        match input.peek() {
            None => continue,
            Some(command_name) => println!(
                "Did not recognize command {}. Enter HELP for help.",
                *command_name
            ),
        };
    }
}
