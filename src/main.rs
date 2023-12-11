use clap::Parser;

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
    node_simulator::run(args.default_texture, !args.no_display);
}
