use super::io::AdventureIO;

pub struct GameInfo<'a> {
    io: AdventureIO,
    game_name: &'a str,
    script_name: &'a str,
    pointer: u32,
}

impl GameInfo<'_> {
    fn create(io: AdventureIO, game: &str) -> GameInfo {
        GameInfo {
            io: io,
            game_name: game,
            script_name: "start",
            pointer: 0,
        }
    }

    fn script_data(&self) -> (&str, u32) { //used for error messages
        (self.script_name, self.pointer+1)
    }

    fn set_pointer(&mut self, pointer: u32) {
        self.pointer = pointer - 1;
    }

    fn next_line(&mut self) {
        self.pointer += 1;
    }
}