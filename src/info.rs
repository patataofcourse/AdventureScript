use super::io::AdventureIO;

pub struct GameInfo {
    io: AdventureIO,
    game_name: String,
    script_name: String,
    pointer: u32,
}

impl GameInfo {
    pub fn create(io: AdventureIO, game: &str) -> GameInfo {
        GameInfo {
            io: io,
            game_name: String::from(game),
            script_name: String::from("start"),
            pointer: 0,
        }
    }

    pub fn script_data(&self) -> (&str, u32) {
        //used for error messages
        (&self.script_name, self.pointer + 1)
    }

    pub fn set_pointer(&mut self, pointer: u32) {
        self.pointer = pointer - 1;
    }

    pub fn next_line(&mut self) {
        self.pointer += 1;
    }

    pub fn get_io(&self) -> &AdventureIO {
        &self.io
    }
}
