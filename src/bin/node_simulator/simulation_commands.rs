pub mod add_command;
pub mod get_command;
pub mod remove_command;
pub mod set_command;
pub mod step_command;

#[derive(clap::Parser, Debug)]
#[command(help_template = "Commands:\r\n{subcommands}")]
pub struct SimulationCommands {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Add(add_command::AddCommand),
    Remove(remove_command::RemoveCommand),
    Set(set_command::SetCommand),
    Get(get_command::GetCommand),
    ToggleScene,
    Close,
    Step(step_command::StepCommand),
}

impl SimulationCommands {
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
