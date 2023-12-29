use clap::{command, Parser};

/// Program for running node-based simulations
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    /// Optional path to a default texture
    #[arg(short, long)]
    pub default_texture: Option<String>,
    #[arg(long, default_value_t = false)]
    pub no_display: bool,
}
