use crate::core::{ASType, CmdSet, Command, GameInfo, TypeMethods};
use std::collections::HashMap;

pub mod inventory;

pub struct Module {
    pub name: String, //TODO: make it settable through config
    pub commands: CmdSet,
    pub objects: Vec<ObjSpec>,
}

impl Module {
    pub fn from(
        name: String,
        commands: Vec<Command>,
        aliases: HashMap<String, String>,
        objects: Vec<ObjSpec>,
    ) -> Self {
        Self {
            name,
            commands: CmdSet::from(commands, aliases),
            objects,
        }
    }

    pub fn add_to(self, info: &mut GameInfo, commands: &mut CmdSet) {
        info.add_module(self.objects, &self.name);
        commands.add_module(self.commands, &self.name)
    }
}

pub struct ObjSpec {
    pub name: String,
    pub methods: TypeMethods,
    pub fields: HashMap<String, ASType>,
}

impl ObjSpec {
    pub fn adapt_for_module(self, module_name: &str) -> Self {
        Self {
            name: format!("{}.{}", module_name, self.name),
            methods: self.methods,
            fields: self.fields,
        }
    }
}
