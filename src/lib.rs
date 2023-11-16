use std::io;
use std::thread;

use clap::ArgMatches;
use graphics::node_events;
use graphics::state;
use node::NodePosition;
use winit::event_loop;

mod graphics;
mod node;
mod node_collection;
mod resources;

mod commands;

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
    let add_command = commands::CommandGenerator::add_command();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let mut input = input.trim().split_ascii_whitespace().peekable();

        match input.peek() {
            None => continue,
            Some(command_name) if *command_name == add_command.get_name() => {
                let args = add_command.clone().try_get_matches_from(input);
                match args {
                    Ok(result) => try_execute_add_command(result, &event_loop_proxy),
                    Err(err) => println!("{}", err),
                }
            }
            Some(command_name) => println!(
                "Did not recognize command {}. Enter HELP for help.",
                *command_name
            ),
        };
    }
}

fn try_execute_add_command(
    args: ArgMatches,
    event_loop_proxy: &event_loop::EventLoopProxy<node_events::NodeEvent>,
) {
    let id = args.get_one::<String>("id").expect("ID arg was missing");
    let id = id.parse::<u32>();
    if id.is_err() {
        println!("ID must be a u32");
        return;
    }
    let id = node::NodeId(id.unwrap());
    let node = node::Node::new(id, node::NodePosition { x: 5, y: 7 });
    let result = event_loop_proxy.send_event(node_events::NodeEvent::Add(node));
    let _ = match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };
}
