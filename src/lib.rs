use std::path::PathBuf;

pub mod config;
pub mod error;

mod commands;
mod info;
mod io;
mod macros;
mod parsing;
pub mod save;
mod variables;

mod inventory {}
mod methods {}

// TODO: (more) public imports for stuff that might be used in the interface
pub use commands::{CmdSet, Command};
pub use info::GameInfo;
pub use io::{AdventureIO, FileType};
pub use variables::{ASType, ASVariable};

pub struct AdventureScriptGame {
    info: GameInfo,
    commands: CmdSet,
}

impl AdventureScriptGame {
    /// document this better later, me
    /// however, root_dir is basically the root folder of the game
    pub fn new(
        root_dir: String,
        io: Option<io::AdventureIO>,
        is_local: bool,
    ) -> AdventureScriptGame {
        AdventureScriptGame {
            info: GameInfo::create(PathBuf::from(root_dir), io.unwrap_or_default(), is_local),
            commands: CmdSet::new(),
        }
    }

    pub fn run(&mut self) {
        //load config file
        if let Err(err) = self.info.load_config() {
            error::manage_error(&self.info, err);
            return;
        };
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
