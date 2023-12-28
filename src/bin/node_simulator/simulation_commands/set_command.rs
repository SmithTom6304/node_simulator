pub mod fps_args;
pub mod node_args;
pub mod tps_args;

use fps_args::FpsArgs;
use node_args::NodeArgs;
use tps_args::TpsArgs;

#[derive(clap::Parser, Debug)]
#[command(help_template = "Commands:\r\n{subcommands}")]
pub struct SetCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Node(NodeArgs),
    Tps(TpsArgs),
    Fps(FpsArgs),
}
