use super::{
    info::GameInfo,
    variables::{ASType, ASVariable},
};
use std::collections::HashMap;

//TODO: figure out how this will work??
pub struct Command {
    name: String,
    func: fn(GameInfo, Vec<&ASVariable>, HashMap<String, &ASVariable>),
    args_to_kwargs: Vec<String>,
    accepted_kwargs: HashMap<String, ASType>,
}

pub fn input(inf: GameInfo, args: Vec<&ASVariable>, kwargs: HashMap<String, &ASVariable>) {
    (inf.get_io().wait)();
}

pub fn choice(inf: GameInfo, args: Vec<&ASVariable>, kwargs: HashMap<String, &ASVariable>) {
    ()
}
