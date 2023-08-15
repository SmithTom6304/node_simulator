use std::io;

mod node;
mod display;
mod graphics;

const QUIT_COMMAND: &str = ":q";

pub fn run() {
    let graphics = graphics::Graphics::new();
    
    println!("Running node_simulator...");
    help();

   read_input(); 
}

fn help() {
    println!("Enter commands via the CLI.");
    println!("Enter :h for help, or :q to quit");
}

fn read_input() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        match input {
            QUIT_COMMAND => break,
            ":h" => help(),
            _ => (),
        }
    }
}

