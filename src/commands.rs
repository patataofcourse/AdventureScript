use super::{
    error::{self, ASErr},
    info::GameInfo,
    variables::{ASType, ASVariable},
};
use anyhow;
use std::{collections::HashMap, iter::FromIterator};

//TODO: figure out how this will work??
pub struct Command {
    pub name: String,
    func: fn(&mut GameInfo, HashMap<String, &ASVariable>) -> anyhow::Result<()>,
    args_to_kwargs: Vec<String>,
    accepted_kwargs: HashMap<String, ASType>,
    default_values: HashMap<String, ASVariable>,
}

impl Command {
    pub fn run<'a>(
        self,
        info: &mut GameInfo,
        args: Vec<&'a ASVariable>,
        kwargs: HashMap<String, &'a ASVariable>,
    ) -> anyhow::Result<()> {
        let (script, line) = info.script_data(); // This will be needed for errors
        let mut c = 0;
        let mut kwargs = kwargs;
        // Turn positional arguments into keyword arguments
        for arg in &args {
            let argname = match self.args_to_kwargs.get(c) {
                None => Err(error::TooManyPositionalArguments {
                    script: String::from(script),
                    line: line,
                    command: String::from(&self.name),
                    max_args: self.args_to_kwargs.len() as u32,
                    given_args: (&args).len() as u32,
                }
                .generic_err()),
                Some(c) => Ok(c),
            }?;
            kwargs.insert(String::from(argname), arg);
            c += 1;
        }
        // Pass default argument values
        for (key, value) in &self.default_values {
            if !kwargs.contains_key(key) {
                kwargs.insert(String::from(key), value);
            }
        }
        // Check that all given arguments are taken by the command and
        // of the required type
        for (key, value) in &kwargs {
            if !self.accepted_kwargs.contains_key(key) {
                Err(error::UndefinedArgument {
                    script: String::from(script),
                    line: line,
                    command: String::from(&self.name),
                    argument_name: String::from(key),
                    argument_type: value.get_type(),
                }
                .generic_err())?;
            }
            let arg_type = value.get_type();
            if self.accepted_kwargs[key] != arg_type {
                Err(error::ArgumentTypeError {
                    script: String::from(script),
                    line: line,
                    command: String::from(&self.name),
                    argument_name: String::from(key),
                    argument_type: &self.accepted_kwargs[key],
                    given_type: value.get_type(),
                }
                .generic_err())?;
            }
        }
        // Check that all arguments in the command have a value
        for (key, value) in &self.accepted_kwargs {
            if !kwargs.contains_key(key) {
                Err(error::MissingRequiredArgument {
                    script: String::from(script),
                    line: line,
                    command: String::from(&self.name),
                    argument_name: String::from(key),
                    argument_type: value,
                }
                .generic_err())?;
            }
        }
        (self.func)(info, kwargs)
    }
}

pub fn input_fn(info: &mut GameInfo, _kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
    (info.get_io().wait)()
}

pub fn choice_fn(info: &mut GameInfo, kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
    let mut a = 1;
    let mut choices = Vec::<&str>::new();
    let mut gotos = Vec::<i32>::new();
    //get lists of the choices and gotos
    while a <= 9 {
        if a == 3 {
            break;
        } //Remove after proper choice command
        let choice = match kwargs[&format!("ch{}", a)] {
            ASVariable::String(c) => c,
            _ => "",
        };
        let goto = match kwargs[&format!("go{}", a)] {
            ASVariable::Int(c) => *c,
            _ => 0,
        };
        if goto == 0 {
            break;
        }
        choices.append(&mut Vec::<&str>::from([choice]));
        gotos.append(&mut Vec::<i32>::from([goto]));
        a += 1;
    }
    //get text
    let text = match kwargs["text"] {
        ASVariable::String(c) => c,
        _ => "",
    };
    //run io func and manage result
    let pick = (info.get_io().query)(text, choices, true)?; //TODO: add allow_save
    if pick == 0 {
        // used in save/return/quit
        info.set_pointer(info.script_data().1 - 1); //TODO: make it possible to get the pointer on its own
        return Ok(());
    };
    info.set_pointer(*gotos.get((pick - 1) as usize).expect(""));
    Ok(())
}

pub fn goto_fn(info: &mut GameInfo, kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
    let pos = match kwargs["pos"] {
        ASVariable::Int(c) => *c,
        _ => 0,
    };
    info.set_pointer(pos);
    Ok(())
}

pub fn ending_fn(info: &mut GameInfo, kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
    let name = match kwargs["name"] {
        ASVariable::String(c) => c,
        _ => "",
    };
    (info.get_io().show)(&format!("Ending: {}", name))?;
    info.quit();
    Ok(())
}

pub fn test_fn(_inf: &mut GameInfo, kwargs: HashMap<String, &ASVariable>) -> anyhow::Result<()> {
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

pub fn choice() -> Command {
    let mut accepted = HashMap::<String, ASType>::with_capacity(5);
    accepted.insert(String::from("text"), ASType::String);
    accepted.insert(String::from("ch1"), ASType::String);
    accepted.insert(String::from("ch2"), ASType::String);
    accepted.insert(String::from("go1"), ASType::Int);
    accepted.insert(String::from("go2"), ASType::Int);
    let default = HashMap::<String, ASVariable>::from_iter([
        (
            String::from("ch1"),
            ASVariable::String("Choice 1".to_string()),
        ),
        (
            String::from("ch2"),
            ASVariable::String("Choice 2".to_string()),
        ),
        (String::from("go1"), ASVariable::Int(0)),
        (String::from("go2"), ASVariable::Int(0)),
        (String::from("text"), ASVariable::String(String::from(""))),
    ]);
    Command {
        name: "test".to_string(),
        func: choice_fn,
        args_to_kwargs: Vec::<String>::new(),
        accepted_kwargs: accepted,
        default_values: default,
    }
}
