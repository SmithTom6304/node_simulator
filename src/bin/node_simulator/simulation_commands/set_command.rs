pub mod node_args;

use node_args::NodeArgs;

#[derive(clap::Parser, Debug)]
#[command(help_template = "Commands:\r\n{subcommands}")]
pub struct SetCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Node(NodeArgs),
}
