pub mod model_args;

use model_args::ModelArgs;

#[derive(clap::Args, Debug)]
#[command(help_template = "Commands:\r\n{subcommands}")]
pub struct LoadCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Model(ModelArgs),
}
