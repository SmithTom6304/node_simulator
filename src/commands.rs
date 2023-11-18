use clap::{Arg, Command};

pub struct CommandGenerator {}

impl CommandGenerator {
    pub fn help_command() -> clap::Command {
        Command::new("--help")
            .subcommand(Self::add_command())
            .subcommand(Self::remove_command())
            .subcommand(Self::close_command())
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

    pub fn remove_command() -> clap::Command {
        let id_arg = Arg::new("id")
            .long("id")
            .help("ID of the node to remove")
            .required(true);

        Command::new("REMOVE")
            .arg(id_arg)
            .about("Remove a node from the simulation")
    }

    pub fn close_command() -> clap::Command {
        Command::new("CLOSE").about("Close the program")
    }
}
