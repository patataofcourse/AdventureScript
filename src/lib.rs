pub mod commands;

pub mod error;

pub mod io;

mod info;

mod inventory {}

mod methods {}

mod parsing;

pub mod variables;

pub struct AdventureScriptGame {
    info: info::GameInfo,
}

impl AdventureScriptGame {
    pub fn run(&self) {
        //TODO: add game running code here
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
