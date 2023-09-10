use std::io;
use std::thread;

use winit::event;
use winit::event_loop;
use winit::event_loop::EventLoopProxy;

use cgmath::prelude::*;

mod graphics;
mod node;
mod node_collection;
mod resources;

const QUIT_COMMAND: &str = ":q";

pub fn run() {
    graphics::init();
    let event_loop = graphics::get_event_loop();
    let event_loop_proxy = event_loop.create_proxy();

    thread::spawn(|| {
        println!("Running node_simulator...");
        help();
        read_input(event_loop_proxy);
    });
    pollster::block_on(graphics::run(event_loop));
}

fn help() {
    println!("Enter commands via the CLI.");
    println!("Enter :h for help, or :q to quit");
}

fn read_input(event_loop_proxy: event_loop::EventLoopProxy<event::WindowEvent>) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        match input {
            QUIT_COMMAND => {
                event_loop_proxy
                    .send_event(event::WindowEvent::CloseRequested)
                    .ok();
                break;
            }
            ":h" => help(),
            _ => (),
        }
    }
}
