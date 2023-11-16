use clap::{Arg, Command};

pub struct CommandGenerator {}

impl CommandGenerator {
    pub fn add_command() -> clap::Command {
        let id_arg = Arg::new("id")
            .long("id")
            .help("ID of the node to add")
            .required(true);

        Command::new("ADD").arg(id_arg)
    }
}
