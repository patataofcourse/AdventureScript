use super::io::AdventureIO;

pub struct GameInfo {
    io: AdventureIO,
    game_name: String,
    script_name: String,
    pointer: i32,
    quitting: bool,
}

impl GameInfo {
    pub fn create(io: AdventureIO, game: String) -> GameInfo {
        GameInfo {
            io: io,
            game_name: game,
            script_name: String::from("start"),
            pointer: 0,
            quitting: false,
        }
    }

    pub fn script_data(&self) -> (&str, i32) {
        //used for error messages
        (&self.script_name, self.pointer + 1)
    }

    pub fn set_pointer(&mut self, pointer: i32) {
        self.pointer = pointer - 1;
    }

    pub fn next_line(&mut self) {
        self.pointer += 1;
    }

    pub fn get_io(&self) -> &AdventureIO {
        &self.io
    }

    pub fn quit(&mut self) {
        self.quitting = true;
    }
}
