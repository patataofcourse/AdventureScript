use super::info::GameInfo;
use std::collections::HashMap;

pub struct Command {
    name: String,
    pub func: fn(GameInfo, HashMap<&str, &str>), //TODO: improve args
}

pub fn input(inf: GameInfo) {
    //, kwargs: HashMap<&str, &str>) {
    (inf.get_io().wait)();
}
