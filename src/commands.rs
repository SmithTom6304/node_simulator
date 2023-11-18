use clap::{Arg, Command};

pub struct CommandGenerator {}

impl CommandGenerator {
    pub fn help_command() -> clap::Command {
        Command::new("--help")
            .subcommand(Self::add_command())
            .disable_help_subcommand(true)
            .help_template("Commands:\n{subcommands}")
    }

    pub fn add_command() -> clap::Command {
        let id_arg = Arg::new("id")
            .long("id")
            .help("ID of the node to add")
            .required(true);
        let position_arg = Arg::new("position")
            .long("position")
            .short('p')
            .help("Comma separated value position of the node")
            .required(false)
            .default_value("0,0");

        Command::new("ADD")
            .arg(id_arg)
            .arg(position_arg)
            .about("Add a node to the simulation")
    }
}
