pub mod commands;

pub mod error;

pub mod io;

mod info;

mod inventory {}

mod methods {}

mod parsing;

pub mod variables;

// TODO: public imports for stuff that might be used in the interface
use std::collections::HashMap;

pub struct AdventureScriptGame {
    info: info::GameInfo,
    commands: HashMap<String, commands::Command>,
}

impl AdventureScriptGame {
    /// document this better later, me
    /// however, root_dir is basically the root folder of the game
    pub fn new(root_dir: String, io: Option<io::AdventureIO>) -> AdventureScriptGame {
        AdventureScriptGame {
            info: info::GameInfo::create(root_dir, io.unwrap_or_default()),
            commands: HashMap::<String, commands::Command>::new(),
        }
    }

    pub fn run(&mut self) {
        println!("AdventureScript v{}\n", env!("CARGO_PKG_VERSION"));
        //add basic commands
        self.commands.extend(commands::main_commands());
        //load script file
        if let Err(err) = self.info.load_script(None) {
            error::manage_error(&self.info, err);
            return;
        };
        //parser and stuff
        while !self.info.quitting {
            match parsing::parse_line(&mut self.info, &self.commands) {
                Ok(_) => (),
                Err(err) => {
                    error::manage_error(&self.info, err);
                    return;
                }
            };
            self.info.next_line();
        }
    }
}
