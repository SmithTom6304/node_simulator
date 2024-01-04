use node_simulator::node;

#[derive(clap::Args, Debug)]
pub struct StepCommand {
    #[arg(default_value = "1")]
    pub number_of_steps: u32,
}

impl From<&StepCommand> for node::event::step::StepEvent {
    fn from(value: &StepCommand) -> Self {
        Self {
            steps: value.number_of_steps,
        }
    }
}
