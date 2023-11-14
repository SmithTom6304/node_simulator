use clap::Parser;

/// Program for running node-based simulations
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CArgs {
    /// Optional path to a default texture
    #[arg(short, long)]
    default_texture: Option<String>,
}

fn main() {
    let args = CArgs::parse();
    node_simulator::run(args.default_texture);
}
