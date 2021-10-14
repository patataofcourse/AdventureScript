use super::{
    error::ASError,
    info::GameInfo,
    variables::{ASType, ASVariable},
};
use anyhow;
use std::collections::HashMap;

//TODO: figure out how this will work??
pub struct Command {
    pub name: String,
    func: fn(&GameInfo, HashMap<String, &ASVariable>) -> anyhow::Result<()>,
    args_to_kwargs: Vec<String>,
    accepted_kwargs: HashMap<String, ASType>, // maybe merge this and required_kwargs
    default_values: HashMap<String, ASVariable>,
}

impl Command {
    pub fn run(
        self,
        info: &GameInfo,
        args: Vec<&ASVariable>,
        kwargs: HashMap<String, &ASVariable>,
    ) -> anyhow::Result<()> {
        (self.func)(info, kwargs)
    }
}

pub fn input(inf: &GameInfo, kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
    (inf.get_io().wait)()
}

pub fn choice(inf: &GameInfo, kwargs: HashMap<String, &ASVariable>) {
    ()
}
