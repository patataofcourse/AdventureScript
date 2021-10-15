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
    pub fn run(&self) {
        println!("AdventureScript v2.0.0-alpha.0\n");

        let test = commands::test();
        let var = ASVariable::String(String::from("hi"));
        let args = Vec::<&ASVariable>::from([&var]);
        let mut kwargs = HashMap::<String, &ASVariable>::new();
        kwargs.insert(String::from("test"), &ASVariable::Int(3));
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
