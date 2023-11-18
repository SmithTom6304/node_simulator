use std::io;
use std::thread;

use clap::ArgMatches;
use graphics::node_events;
use graphics::state;
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
    let state = pollster::block_on(state::State::new(window, default_texture_path));

    thread::spawn(|| {
        println!("Running node_simulator...");
        read_input(event_loop_proxy);
    });
    pollster::block_on(graphics::run(event_loop, state));
}

fn read_input(event_loop_proxy: event_loop::EventLoopProxy<node_events::NodeEvent>) {
    let mut help_command = commands::CommandGenerator::help_command();
    let add_command = commands::CommandGenerator::add_command();
    let remove_command = commands::CommandGenerator::remove_command();
    let close_command = commands::CommandGenerator::close_command();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let mut input = input.trim().split_ascii_whitespace().peekable();

        match input.peek() {
            None => continue,
            Some(command_name) if *command_name == help_command.get_name() => {
                let _ = help_command.print_help();
            }
            Some(command_name) if *command_name == add_command.get_name() => {
                let args = add_command.clone().try_get_matches_from(input);
                match args {
                    Ok(result) => try_execute_add_command(result, &event_loop_proxy),
                    Err(err) => println!("{}", err),
                }
            }
            Some(command_name) if *command_name == remove_command.get_name() => {
                let args = remove_command.clone().try_get_matches_from(input);
                match args {
                    Ok(result) => try_execute_remove_command(result, &event_loop_proxy),
                    Err(err) => println!("{}", err),
                }
            }
            Some(command_name) if *command_name == close_command.get_name() => {
                let result = event_loop_proxy.send_event(node_events::NodeEvent::Close);
                let _ = match result {
                    Ok(_) => (),
                    Err(err) => println!("{}", err),
                };
            }
            Some(command_name) => println!(
                "Did not recognize command {}. Enter {} for help.",
                *command_name,
                help_command.get_name()
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

    let position = args.get_one::<String>("position");
    let position = match position {
        Some(pos_string) => {
            let pos_string = pos_string.split(',');
            let positions: Vec<Result<i32, std::num::ParseIntError>> =
                pos_string.map(|s| s.parse::<i32>()).collect();
            if positions.len() != 2 {
                println!("Position must have 2 values");
                return;
            }
            let x = match &positions[0] {
                Ok(number) => *number,
                Err(_) => {
                    println!("Position x must be an i32");
                    return;
                }
            };
            let y = match &positions[1] {
                Ok(number) => *number,
                Err(_) => {
                    println!("Position y must be an i32");
                    return;
                }
            };
            node::NodePosition { x, y }
        }
        None => node::NodePosition { x: 0, y: 0 },
    };

    let node = node::Node::new(id, position);
    let result = event_loop_proxy.send_event(node_events::NodeEvent::Add(node));
    let _ = match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };
}

fn try_execute_remove_command(
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
    let result = event_loop_proxy.send_event(node_events::NodeEvent::Remove(id));
    let _ = match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };
}
