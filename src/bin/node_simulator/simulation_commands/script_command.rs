use std::fs;

use super::SimulationCommand;

#[derive(clap::Args, Debug)]
pub struct ScriptCommand {
    pub file: String,
}

impl ScriptCommand {
    pub fn load_script(file: String) -> anyhow::Result<Vec<SimulationCommand>> {
        let contents = fs::read_to_string(file)?;
        let lines = contents.lines();
        let commands = lines
            .into_iter()
            .map(|line| SimulationCommand::try_from(line.to_string()))
            .collect::<Result<Vec<_>, _>>();
        match commands {
            Ok(commands) => Ok(commands),
            Err(err) => anyhow::bail!(err),
        }
    }
}
