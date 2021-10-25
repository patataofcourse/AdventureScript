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

pub struct AdventureScriptGame {
    info: info::GameInfo,
    commands: Vec<commands::Command>,
}

impl AdventureScriptGame {
    pub fn run(&mut self) {
        println!("AdventureScript v2.0.0-alpha.0\n");
        //add basic commands
        self.commands.extend(commands::main_commands());
        //parser and stuff
        while !self.info.quitting() {
            match parsing::basic_script(&mut self.info, &self.commands) {
                Ok(_) => (),
                Err(err) => {
                    print!(
                        "\nAdventureScript error on script {}, line {} - ",
                        self.info.script_name(),
                        self.info.pointer(),
                    );
                    //TODO: this sucks ass pls fix (ASError enum?)
                    if let Some(_c) = err.downcast_ref::<error::ASFileError>() {
                    } else if let Some(_c) = err.downcast_ref::<error::ASCmdError>() {
                    } else {
                        print!("uncaught internal error\n\t");
                    };
                    println!("{}", err);
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
        commands: Vec::<commands::Command>::new(),
    }
}
