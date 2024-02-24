pub mod add_command;
pub mod get_command;
pub mod load_command;
pub mod remove_command;
pub mod script_command;
pub mod set_command;
pub mod step_command;

#[derive(clap::Parser, Debug)]
#[command(help_template = "Commands:\r\n{subcommands}")]
pub struct SimulationCommand {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Add(add_command::AddCommand),
    Remove(remove_command::RemoveCommand),
    Set(set_command::SetCommand),
    Get(get_command::GetCommand),
    ToggleScene,
    Close,
    Step(step_command::StepCommand),
    Script(script_command::ScriptCommand),
    Load(load_command::LoadCommand),
}

impl TryFrom<String> for SimulationCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut value = value.to_lowercase().trim().to_string();
        // Insert dummy char for command parsing
        value.insert(0, ' ');
        value.insert(0, '@');

        use clap::Parser;
        let command: Result<SimulationCommand, clap::error::Error> =
            Self::try_parse_from(value.split_whitespace());
        match command {
            Ok(command) => Ok(command),
            Err(e) => Err(SimulationCommand::remove_dummy_char_from_usage_string(
                e.to_string(),
            )),
        }
    }
}

impl SimulationCommand {
    pub fn remove_dummy_char_from_usage_string(message: String) -> String {
        message
            .lines()
            .map(|line| {
                let mut line = line.replace("@ ", "");
                line.push('\n');
                line
            })
            .collect()
    }
}
