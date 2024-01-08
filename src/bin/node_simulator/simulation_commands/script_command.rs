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
            .map(|line| line.trim())
            .map(Self::remove_comments)
            .filter(|line| !line.is_empty())
            .map(|line| SimulationCommand::try_from(line.to_string()))
            .collect::<Result<Vec<_>, _>>();
        match commands {
            Ok(commands) => Ok(commands),
            Err(err) => anyhow::bail!(err),
        }
    }

    fn remove_comments(line: &str) -> &str {
        if !line.contains("//") {
            return line;
        }
        return line.split("//").next().unwrap_or_default();
    }
}
