use std::sync::{mpsc, Arc, Mutex};
use std::time::{self, Duration};
use std::{io, thread};

use clap::{ArgMatches, Parser};

use node_simulator::graphics::{self, scene_event};
use node_simulator::{commands, node, simulation};

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
    let simulation = Arc::new(Mutex::new(simulation::Simulation::new()));
    let (simulation_tx, simulation_rx) = mpsc::channel::<Arc<Mutex<simulation::Simulation>>>();
    let (scene_event_tx, scene_event_rx) = mpsc::channel::<scene_event::SceneEvent>();
    let (node_event_tx, node_event_rx) = mpsc::channel::<node::Event>();

    let graphics_interface = graphics::GraphicsInterface::new(simulation_rx, create_display);

    thread::spawn(move || {
        let simulation = Arc::clone(&simulation);
        run_simulation(simulation, simulation_tx, node_event_rx);
    });

    thread::spawn(|| {
        println!("Running node_simulator...");
        read_input(scene_event_tx, node_event_tx);
    });
    graphics_interface.run(scene_event_rx);
}

fn run_simulation(
    simulation: Arc<Mutex<simulation::Simulation>>,
    simulation_tx: mpsc::Sender<Arc<Mutex<simulation::Simulation>>>,
    node_event_rx: mpsc::Receiver<node::Event>,
) {
    //TODO Lowered for debugging purposes
    let target_fps = 2;
    let target_duration = Duration::new(1, 0) / target_fps;
    let print_poor_performance = false;

    loop {
        let event = node_event_rx.try_recv();
        let start_time = time::Instant::now();
        {
            let mut sim = simulation.lock().unwrap();

            match event {
                Ok(event) => sim.handle_event(event),
                Err(_) => {}
            }
            sim.step();
        }

        let _ = simulation_tx.send(simulation.clone());

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

fn read_input(
    scene_event_tx: mpsc::Sender<scene_event::SceneEvent>,
    node_event_tx: mpsc::Sender<node::Event>,
) {
    let mut help_command = commands::CommandGenerator::help_command();
    let add_command = commands::CommandGenerator::add_command();
    let remove_command = commands::CommandGenerator::remove_command();
    let close_command = commands::CommandGenerator::close_command();
    let toggle_scene_command = commands::CommandGenerator::toggle_scene_command();
    let set_target_fps_command = commands::CommandGenerator::target_fps_command();
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
                        _ = node_event_tx.send(node::Event {
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
                        _ = node_event_tx.send(node::Event {
                            remove_node_event: try_execute_remove_command(result),
                            ..Default::default()
                        })
                    }
                    Err(err) => println!("{}", err),
                }
            }
            Some(command_name) if *command_name == close_command.get_name() => {
                let result = scene_event_tx.send(scene_event::CloseEvent::new());
                let _ = match result {
                    Ok(_) => (),
                    Err(err) => println!("{}", err),
                };
            }
            Some(command_name) if *command_name == toggle_scene_command.get_name() => {
                let result = scene_event_tx.send(scene_event::ToggleSceneEvent::new());
                let _ = match result {
                    Ok(_) => (),
                    Err(err) => println!("{}", err),
                };
            }
            Some(command_name) if *command_name == set_target_fps_command.get_name() => {
                let args = set_target_fps_command
                    .clone()
                    .try_get_matches_from(input)
                    .unwrap();
                let fps = match args.get_one::<String>("target_fps") {
                    Some(fps) => match fps.parse::<u32>() {
                        Ok(fps) => Some(fps),
                        Err(_) => {
                            println!("Target fps must be a u32");
                            None
                        }
                    },
                    None => None,
                };
                let result = scene_event_tx.send(scene_event::SetTargetFpsEvent::new(fps));
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

fn try_execute_add_command(args: ArgMatches) -> Option<node::AddNodeEvent> {
    let id = args.get_one::<String>("id").expect("ID arg was missing");
    let id = id.parse::<u32>();
    if id.is_err() {
        println!("ID must be a u32");
        return None;
    }
    let id = node::Id(id.unwrap());

    let position = args.get_one::<String>("position");
    let position = match position {
        Some(pos_string) => {
            let pos_string = pos_string.split(',');
            let positions: Vec<Result<i32, std::num::ParseIntError>> =
                pos_string.map(|s| s.parse::<i32>()).collect();
            if positions.len() != 3 {
                println!("Position must have 3 values");
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
            let z = match &positions[2] {
                Ok(number) => *number,
                Err(_) => {
                    println!("Position z must be an i32");
                    return None;
                }
            };
            node::Position { x, y, z }
        }
        None => node::Position { x: 0, y: 0, z: 0 },
    };

    let node = node::Node::new(id, position);
    Some(node::AddNodeEvent { node })
}

fn try_execute_remove_command(args: ArgMatches) -> Option<node::RemoveNodeEvent> {
    let id = args.get_one::<String>("id").expect("ID arg was missing");
    let id = id.parse::<u32>();
    if id.is_err() {
        println!("ID must be a u32");
        return None;
    }
    let id = node::Id(id.unwrap());
    Some(node::RemoveNodeEvent { node_id: id })
}
