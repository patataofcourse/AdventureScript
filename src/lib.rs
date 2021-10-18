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
use variables::ASVariable;

pub struct AdventureScriptGame {
    info: info::GameInfo,
}

impl AdventureScriptGame {
    pub fn run(&mut self) {
        println!("AdventureScript v2.0.0-alpha.0\n");
        //TODO: parser and stuff :D
        while !self.info.quitting() {
            match parsing::basic_script(&mut self.info) {
                Ok(c) => (),
                Err(c) => {
                    println!("{}", c);
                    break;
                }
            };
            self.info.next_line();
        }
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
