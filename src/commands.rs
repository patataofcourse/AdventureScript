use super::{
    error::{ASError},
    info::GameInfo,
    variables::{ASType, ASVariable},
};
use std::collections::HashMap;

//TODO: figure out how this will work??
pub struct Command {
    name: String,
    func: fn(GameInfo, HashMap<String, &ASVariable>) -> Result<(), ASError>,
    args_to_kwargs: Vec<String>,
    accepted_kwargs: HashMap<String, ASType>,
}

impl Command {
    pub fn run(self, info: GameInfo, args: Vec<String>, kwargs: HashMap<String, &ASVariable>) -> Result<(), ASError>{
        (self.func)(info, kwargs)
    }
}

pub fn input(inf: GameInfo, kwargs: HashMap<String, &ASVariable>) -> Result<(), ASError> {
    (inf.get_io().wait)()
}

pub fn choice(inf: GameInfo, kwargs: HashMap<String, &ASVariable>) {
    ()
}
