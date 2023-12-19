use std::sync::mpsc;

use clap::Parser;
use node_simulator::commands;
use node_simulator::graphics;
use node_simulator::node;
use node_simulator::simulation;

use std::io;

use std::thread;

use clap::ArgMatches;
use node_simulator::graphics::node_events;

/// Program for running node-based simulations
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CArgs {
    /// Optional path to a default texture
    #[arg(short, long)]
    default_texture: Option<String>,
    #[arg(long, default_value_t = false)]
    no_display: bool,
}

fn main() {
    let args = CArgs::parse();
    run(args.default_texture, !args.no_display);
}

pub fn run(_default_texture_path: Option<String>, create_display: bool) {
    let mut simulation = simulation::Simulation::new();
    let graphics_interface = graphics::GraphicsInterface::new(&mut simulation, create_display);

    let (tx, rx) = mpsc::channel::<node_events::NodeEvent>();
    thread::spawn(|| {
        println!("Running node_simulator...");
        read_input(tx);
    });
    graphics_interface.run(rx);
}

fn read_input(tx: mpsc::Sender<node_events::NodeEvent>) {
    let mut help_command = commands::CommandGenerator::help_command();
    let add_command = commands::CommandGenerator::add_command();
    let remove_command = commands::CommandGenerator::remove_command();
    let close_command = commands::CommandGenerator::close_command();
    let toggle_scene_command = commands::CommandGenerator::toggle_scene_command();
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
                    Ok(result) => {
                        _ = tx.send(node_events::NodeEvent {
                            add_node_event: try_execute_add_command(result),
                            ..Default::default()
                        })
                    }
                    Err(err) => println!("{}", err),
                }
            }
            Some(command_name) if *command_name == remove_command.get_name() => {
                let args = remove_command.clone().try_get_matches_from(input);
                match args {
                    Ok(result) => {
                        _ = tx.send(node_events::NodeEvent {
                            remove_node_event: try_execute_remove_command(result),
                            ..Default::default()
                        })
                    }
                    Err(err) => println!("{}", err),
                }
            }
            Some(command_name) if *command_name == close_command.get_name() => {
                let result = tx.send(node_events::CloseEvent::new());
                let _ = match result {
                    Ok(_) => (),
                    Err(err) => println!("{}", err),
                };
            }
            Some(command_name) if *command_name == toggle_scene_command.get_name() => {
                let result = tx.send(node_events::ToggleSceneEvent::new());
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

fn try_execute_add_command(args: ArgMatches) -> Option<node_events::AddNodeEvent> {
    let id = args.get_one::<String>("id").expect("ID arg was missing");
    let id = id.parse::<u32>();
    if id.is_err() {
        println!("ID must be a u32");
        return None;
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
                return None;
            }
            let x = match &positions[0] {
                Ok(number) => *number,
                Err(_) => {
                    println!("Position x must be an i32");
                    return None;
                }
            };
            let y = match &positions[1] {
                Ok(number) => *number,
                Err(_) => {
                    println!("Position y must be an i32");
                    return None;
                }
            };
            node::NodePosition { x, y }
        }
        None => node::NodePosition { x: 0, y: 0 },
    };

    let node = node::Node::new(id, position);
    Some(node_events::AddNodeEvent { node })
}

fn try_execute_remove_command(args: ArgMatches) -> Option<node_events::RemoveNodeEvent> {
    let id = args.get_one::<String>("id").expect("ID arg was missing");
    let id = id.parse::<u32>();
    if id.is_err() {
        println!("ID must be a u32");
        return None;
    }
    let id = node::NodeId(id.unwrap());
    Some(node_events::RemoveNodeEvent { node_id: id })
}
