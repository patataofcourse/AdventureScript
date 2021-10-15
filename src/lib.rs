pub mod commands;

pub mod error;

pub mod io;

mod info;

mod inventory {}

mod methods {}

mod parsing;

pub mod variables;

use std::collections::HashMap;
use variables::{ASType, ASVariable};

pub struct AdventureScriptGame {
    info: info::GameInfo,
}

impl AdventureScriptGame {
    pub fn run(&self) {
        println!("AdventureScript v2.0.0-alpha.0\n");

        let test = commands::test();
        let args = Vec::<&ASVariable>::new();
        let kwargs = HashMap::<String, &ASVariable>::new();
        let result = test.run(&self.info, args, kwargs);
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
