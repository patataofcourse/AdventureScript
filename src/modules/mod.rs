use crate::core::{ASType, ASVariable, CmdSet, Command, GameInfo, TypeMethods};
use std::collections::HashMap;

pub mod inventory;

pub struct Module {
    pub name: String, //TODO: make it settable through config
    pub commands: CmdSet,
    pub objects: Vec<ObjSpec>,
    pub globals: HashMap<String, ASType>,
}

impl Module {
    pub fn from(
        name: String,
        commands: Vec<Command>,
        aliases: HashMap<String, String>,
        objects: Vec<ObjSpec>,
        globals: HashMap<String, ASType>,
    ) -> Self {
        Self {
            name,
            commands: CmdSet::from(commands, aliases),
            objects,
            globals,
        }
    }

    pub fn add_to(self, info: &mut GameInfo, commands: &mut CmdSet) {
        info.add_module(self.objects, self.globals, &self.name);
        commands.add_module(self.commands, &self.name)
    }
}

#[derive(Clone)]
pub struct ObjSpec {
    pub name: String,
    pub methods: TypeMethods,
    pub fields: HashMap<String, ASType>,
    pub stringify: fn(HashMap<String, ASVariable>) -> String,
}

impl ObjSpec {
    pub fn adapt_for_module(self, module_name: &str) -> Self {
        Self {
            name: format!("{}.{}", module_name, self.name),
            methods: self.methods,
            fields: self.fields,
            stringify: self.stringify,
        }
    }
}
