pub mod node_args;

use node_args::NodeArgs;

#[derive(clap::Parser, Debug)]
#[command(help_template = "Commands:\r\n{subcommands}")]
pub struct GetCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Node(NodeArgs),
    Tps,
    Fps,
}
