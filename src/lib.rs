//! AdventureScript is a crate for creating text-based games.
//!
//! If you just want to run a game, the [AdventureScriptGame] struct handles all needed processes.
//! Example:
//! ```no_run
//! let mut game = AdventureScriptGame::new("path_to_game".to_string(), None);
//! game.run();
//! ```
//!
//! If what you want is to make a module, you can find the public API for the AdventureScript core in
//! the [core] module, and the macros available in the crate will help keep your code readable. [modules]
//! has both a module API and the built-in modules as examples.

pub mod core;
pub mod modules;

pub(crate) mod formats;
mod macros;
mod parsing;
mod inventory {}

use crate::core::{error::manage_error, main_commands, AdventureIO, CmdSet, GameInfo};
use semver::Version;
use std::path::PathBuf;

pub fn get_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

/// A struct that handles initializing and running an AdventureScript game.
pub struct AdventureScriptGame {
    info: GameInfo,
    commands: CmdSet,
}

impl AdventureScriptGame {
    /// Create a new AdventureScript runner
    ///
    /// * `root_dir` - Root folder of the game's data
    /// * `io` - The IO system to use when running the game
    /// * `is_local` - Whether this game is ran as "portable". If true, saves will be stored in
    /// the ./save folder, otherwise, in AppData, ~/.config, or equivalent
    /// * `is_debug` - Whether to run the game in debug mode or not. Debug mode shows some
    /// extra warnings, like deprecations
    pub fn new(
        root_dir: String,
        io: Option<AdventureIO>,
        is_local: bool,
        is_debug: bool,
    ) -> AdventureScriptGame {
        AdventureScriptGame {
            info: GameInfo::create(
                PathBuf::from(root_dir),
                io.unwrap_or_default(),
                is_local,
                is_debug,
            ),
            commands: CmdSet::new(),
        }
    }

    pub fn run(&mut self) {
        //load config file
        if let Err(err) = self.info.load_config() {
            manage_error(&self.info, err);
            return;
        };
        if self.info.debug {
            println!("AdventureScript v{}\n", env!("CARGO_PKG_VERSION"));
        }
        //add basic commands
        self.commands
            .extend(main_commands().expect("Main commands should not panic"));
        //load script file
        if let Err(err) = self.info.load_script(None) {
            manage_error(&self.info, err);
            return;
        };
        //main loop
        while !self.info.quitting {
            match parsing::parse_line(&mut self.info, &self.commands) {
                Ok(_) => (),
                Err(err) => {
                    manage_error(&self.info, err);
                    return;
                }
            };
            self.info.next_line();
        }
    }

    pub fn add_module(&mut self, module: modules::Module) -> anyhow::Result<()> {
        //TODO: error if module already exists
        module.add_to(&mut self.info, &mut self.commands);
        Ok(())
    }
}
