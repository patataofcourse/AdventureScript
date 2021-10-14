use super::info::GameInfo;
use std::collections::HashMap;

//TODO: figure out how this will work??
pub struct Command {
    name: String,
    pub func: fn(GameInfo, HashMap<&str, &str>), //TODO: improve args
}

pub fn input(inf: GameInfo) {
    //, kwargs: HashMap<&str, &str>) {
    (inf.get_io().wait)();
}

pub fn choice(inf: GameInfo, args: Vec<&str>, kwargs: HashMap<&str, &str>) {
    ()
}
