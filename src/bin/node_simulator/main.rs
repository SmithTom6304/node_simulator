use std::sync::{mpsc, Arc, Mutex};
use std::time::{self, Duration};
use std::{io, thread};

use clap::Parser;

use node_simulator::graphics::scene_event::{CloseEvent, ToggleSceneEvent};
use node_simulator::graphics::{self, scene_event, GraphicsInterface};
use node_simulator::{node, simulation};

use args::CLIArgs;
use simulation_commands::script_command::ScriptCommand;
use simulation_commands::SimulationCommand;

mod args;
mod simulation_commands;

fn main() {
    let args = CLIArgs::parse();
    run(args.default_texture, !args.no_display);
}

fn create_graphics_interface(
    simulation_rx: mpsc::Receiver<Arc<Mutex<simulation::Simulation>>>,
    create_display: bool,
) -> GraphicsInterface {
    graphics::GraphicsInterface::new(simulation_rx, create_display)
}

fn run(_default_texture_path: Option<String>, create_display: bool) {
    let simulation = Arc::new(Mutex::new(simulation::Simulation::new()));
    let (simulation_tx, simulation_rx) = mpsc::channel::<Arc<Mutex<simulation::Simulation>>>();
    let (scene_event_tx, scene_event_rx) = mpsc::channel::<scene_event::Event>();
    let (node_event_tx, node_event_rx) = mpsc::channel::<node::Event>();

    let graphics_interface = create_graphics_interface(simulation_rx, create_display);

    thread::spawn(move || {
        let simulation = Arc::clone(&simulation);
        run_simulation(simulation, simulation_tx, node_event_rx);
    });

    thread::spawn(|| {
        println!("Running node_simulator...");
        read_input_from_cli(scene_event_tx, node_event_tx);
    });
    graphics_interface.run(scene_event_rx);
}

pub fn run_simulation(
    simulation: Arc<Mutex<simulation::Simulation>>,
    simulation_tx: mpsc::Sender<Arc<Mutex<simulation::Simulation>>>,
    node_event_rx: mpsc::Receiver<node::Event>,
) {
    let get_target_duration = |target_tps| Duration::new(1, 0) / target_tps;
    let target_duration_if_paused = Duration::new(1, 0);
    let mut target_duration;
    let print_poor_performance = false;

    loop {
        let event = node_event_rx.try_recv();
        let start_time = time::Instant::now();
        {
            let mut sim = simulation.lock().unwrap();
            let target_tps = sim.target_tps();
            let sim_is_paused = target_tps == 0;
            target_duration = match sim_is_paused {
                true => target_duration_if_paused,
                false => get_target_duration(target_tps),
            };
            if let Ok(event) = event {
                sim.handle_event(event)
            }

            if !sim_is_paused {
                sim.step();
            }
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

fn read_input_from_cli(
    scene_event_tx: mpsc::Sender<scene_event::Event>,
    node_event_tx: mpsc::Sender<node::Event>,
) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let command = SimulationCommand::try_from(input);
        match command {
            Ok(command) => execute_command(command, &scene_event_tx, &node_event_tx),
            Err(e) => println!(
                "{}",
                SimulationCommand::remove_dummy_char_from_usage_string(e.to_string())
            ),
        }
    }
}

fn execute_command(
    simulation_command: SimulationCommand,
    scene_event_tx: &mpsc::Sender<scene_event::Event>,
    node_event_tx: &mpsc::Sender<node::Event>,
) {
    match &simulation_command.command {
        simulation_commands::Command::Add(args) => match &args.command {
            simulation_commands::add_command::Commands::Node(node_args) => {
                let add_event = node::AddNodeEvent::try_from(node_args);
                match add_event {
                    Ok(add_event) => _ = node_event_tx.send(node::Event::AddNode(add_event)),
                    Err(err) => println!("{}", err),
                }
            }
        },
        simulation_commands::Command::Remove(args) => match &args.command {
            simulation_commands::remove_command::Commands::Node(node_args) => {
                _ = node_event_tx.send(node::Event::RemoveNode(node::RemoveNodeEvent::from(
                    node_args,
                )))
            }
        },
        simulation_commands::Command::ToggleScene => {
            _ = scene_event_tx.send(scene_event::Event::ToggleScene(ToggleSceneEvent {}))
        }
        simulation_commands::Command::Close => {
            _ = scene_event_tx.send(scene_event::Event::Close(CloseEvent {}))
        }
        simulation_commands::Command::Set(set_args) => match &set_args.command {
            simulation_commands::set_command::Commands::Node(node_args) => {
                let event = match node::event::set_node::SetNodeEvent::try_from(node_args) {
                    Ok(args) => args,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };
                _ = node_event_tx.send(node::Event::SetNode(event))
            }
            simulation_commands::set_command::Commands::Fps(fps_args) => {
                _ = scene_event_tx.send(scene_event::Event::SetTargetFps(
                    scene_event::SetTargetFpsEvent::from(fps_args),
                ))
            }
            simulation_commands::set_command::Commands::Tps(tps_args) => {
                _ = node_event_tx.send(node::Event::SetTargetTps(node::SetTargetTpsEvent::from(
                    tps_args,
                )))
            }
        },
        simulation_commands::Command::Get(get_args) => match &get_args.command {
            simulation_commands::get_command::Commands::Node(get_node_event) => {
                _ = node_event_tx.send(node::Event::Get(node::event::get::GetEvent::Node(
                    get_node_event.into(),
                )))
            }
            simulation_commands::get_command::Commands::Tps => {
                _ = node_event_tx.send(node::Event::Get(node::event::get::GetEvent::Tps))
            }
            simulation_commands::get_command::Commands::Fps => {
                _ = scene_event_tx.send(scene_event::Event::GetFps)
            }
        },
        simulation_commands::Command::Step(step_args) => {
            _ = node_event_tx.send(node::Event::Step(step_args.into()))
        }
        simulation_commands::Command::Script(script_args) => {
            let commands = match ScriptCommand::load_script(script_args.file.clone()) {
                Ok(commands) => commands,
                Err(err) => {
                    println!("Error running script - {}", err);
                    return;
                }
            };
            for command in commands.into_iter() {
                execute_command(command, scene_event_tx, node_event_tx)
            }
        }
    }
}
