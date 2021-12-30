use crate::core::{CmdSet, Command, GameInfo};
use std::collections::HashMap;

pub mod inventory;

pub struct Module {
    pub name: String,
    pub commands: CmdSet,
}

impl Module {
    pub fn from(name: String, commands: Vec<Command>, aliases: HashMap<String, String>) -> Self {
        Self {
            name,
            commands: CmdSet::from(commands, aliases),
        }
    }

    pub fn add_to(&self, _info: &mut GameInfo, commands: &mut CmdSet) -> anyhow::Result<()> {
        commands.add_module(self)
    }
}
