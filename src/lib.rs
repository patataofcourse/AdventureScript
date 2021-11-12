pub mod commands;

pub mod error;

pub mod io;

mod info;

mod inventory {}

mod methods {}

mod parsing;

pub mod variables;

#[cfg(test)]
mod tests;

// TODO: public imports for stuff that might be used in the interface
use std::collections::HashMap;

static VERSION: &str = "2.0.0-alpha.1";

pub struct AdventureScriptGame {
    info: info::GameInfo,
    commands: HashMap<String, commands::Command>,
}

impl AdventureScriptGame {
    pub fn run(&mut self) {
        println!("AdventureScript v{version}\n", version = VERSION);
        //add basic commands
        self.commands.extend(commands::main_commands());
        //load script file
        if let Err(err) = self.info.load_script("start") {
            error::manage_error(&self.info, err);
            return;
        };
        //parser and stuff
        while !self.info.quitting() {
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

pub fn create_game(game_root: String, io: Option<io::AdventureIO>) -> AdventureScriptGame {
    let io = match io {
        None => io::AdventureIO::default(),
        Some(i) => i,
    };
    AdventureScriptGame {
        info: info::GameInfo::create(game_root, io),
        commands: HashMap::<String, commands::Command>::new(),
    }
}
