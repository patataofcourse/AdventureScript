use super::{
    error::{self, ASErr},
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
    accepted_kwargs: HashMap<String, ASType>,
    default_values: HashMap<String, ASVariable>,
}

impl Command {
    pub fn run<'a>(
        self,
        info: &GameInfo,
        args: Vec<&'a ASVariable>,
        kwargs: HashMap<String, &'a ASVariable>,
    ) -> anyhow::Result<()> {
        let mut c = 0;
        let mut kwargs = kwargs;
        // Turn given arguments into keyword arguments
        for arg in &args {
            let argname = match self.args_to_kwargs.get(c) {
                None => {
                    let (script, line) = info.script_data();
                    Err(error::TooManyPositionalArguments {
                        script: String::from(script),
                        line: line,
                        command: String::from(&self.name),
                        max_args: self.args_to_kwargs.len() as u32,
                        given_args: (&args).len() as u32,
                    }
                    .generic_err())
                }
                Some(c) => Ok(c),
            }?;
            kwargs.insert(String::from(argname), arg);
            c += 1;
        }
        (self.func)(info, kwargs)
    }
}

pub fn input(inf: &GameInfo, _kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
    (inf.get_io().wait)()
}

// pub fn choice(inf: &GameInfo, kwargs: HashMap<String, &ASVariable>) {}

pub fn test_fn(_inf: &GameInfo, kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
    for (key, arg) in kwargs {
        println!("{}: {:?}", key, arg);
    }
    Ok(())
}
//TODO: create command list and pass to the info

pub fn test() -> Command {
    let mut accepted = HashMap::<String, ASType>::with_capacity(3);
    accepted.insert(String::from("test"), ASType::Int);
    accepted.insert(String::from("arg1"), ASType::Any);
    accepted.insert(String::from("arg2"), ASType::Any);
    let mut default = HashMap::<String, ASVariable>::with_capacity(1);
    default.insert(
        String::from("arg2"),
        ASVariable::String(String::from("this is a test")),
    );
    Command {
        name: "test".to_string(),
        func: test_fn,
        args_to_kwargs: Vec::<String>::from([String::from("arg1"), String::from("arg2")]),
        accepted_kwargs: accepted,
        default_values: default,
    }
}
