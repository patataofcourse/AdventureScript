use std::collections::HashMap;

pub mod commands;

pub mod error;

pub mod io;

mod info;

mod inventory {}

mod methods {}

mod parsing;

pub mod variables;

use error::ASErr;
use std::error::Error;
use variables::ASVariable;

pub struct AdventureScriptGame {
    info: info::GameInfo,
}

impl AdventureScriptGame {
    pub fn run(&self) {
        let result = commands::input(&self.info, HashMap::<String, &ASVariable>::new());
        if let Err(e) = result {
            println!("{}", e);
        }
        //TODO: parser and stuff :D
    }
}

pub fn create_game(game_name: String, io: Option<io::AdventureIO>) -> AdventureScriptGame {
    let io = match io {
        None => io::AdventureIO::default(),
        Some(i) => i,
    };
    AdventureScriptGame {
        info: info::GameInfo::create(io, game_name),
    }
}
