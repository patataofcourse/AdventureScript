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

    pub fn script_name(&self) -> &str {
        &self.script_name
    }

    pub fn pointer(&self) -> i32 {
        self.pointer + 1
    }

    pub fn set_pointer(&mut self, pointer: i32) {
        self.pointer = pointer - 2;
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
    pub fn quitting(&self) -> bool {
        self.quitting
    }

    //TODO: customization of choice text formatting
    pub fn query(&self, text: &str, choices: Vec<&str>, allow_save: bool) -> anyhow::Result<u8> {
        if !text.is_empty() {
            (self.io.show)(&text)?;
        }
        let mut c = 1;
        for ch in &choices {
            (self.io.show)(&format!("{}. {}", c, ch))?;
            c += 1;
        }
        loop {
            let result = (self.io.input)()?;
            match result.trim() {
                "s" => {
                    if allow_save {
                        (self.io.show)("Would save here")?;
                    }
                }
                "r" => {
                    if allow_save {
                        (self.io.show)("Would restore here")?;
                    }
                }
                "q" => return Ok(0),
                _ => (),
            }
            let num_result: u8 = match result.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if (num_result as usize) <= choices.len() {
                return Ok(num_result);
            }
        }
    }
}
