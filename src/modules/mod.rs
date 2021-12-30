use crate::core::{CmdSet, Command, GameInfo};
use std::collections::HashMap;

pub mod inventory;

pub struct Module {
    pub commands: CmdSet,
}

impl Module {
    pub fn from(commands: Vec<Command>, aliases: HashMap<String, String>) -> Self {
        Self {
            commands: CmdSet::from(commands, aliases),
        }
    }

    pub fn add_to(&self, _info: &mut GameInfo, commands: &mut CmdSet) {
        commands.extend(self.commands.clone());
    }
}
