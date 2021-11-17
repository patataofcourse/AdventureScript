use super::{
    error::{ASCmdError, CommandErrors},
    info::GameInfo,
    variables::{ASType, ASVariable},
};
use anyhow;
use std::{collections::HashMap, iter::FromIterator};

pub struct Command {
    pub name: String,
    func: fn(&mut GameInfo, HashMap<String, ASVariable>) -> anyhow::Result<()>,
    args_to_kwargs: Vec<String>,
    accepted_kwargs: HashMap<String, ASType>,
    default_values: HashMap<String, ASVariable>,
}

impl Command {
    pub fn run(
        &self,
        info: &mut GameInfo,
        args: Vec<ASVariable>,
        kwargs: HashMap<String, ASVariable>,
    ) -> anyhow::Result<()> {
        let mut c = 0;
        let mut kwargs = kwargs;
        // Turn positional arguments into keyword arguments
        for arg in &args {
            let argname = match self.args_to_kwargs.get(c) {
                None => Err(ASCmdError {
                    command: String::from(&self.name),
                    details: CommandErrors::TooManyPosArgs {
                        max_args: self.args_to_kwargs.len() as u32,
                        given_args: (&args).len() as u32,
                    },
                }),
                Some(c) => Ok(c),
            }?;
            kwargs.insert(String::from(argname), arg.to_owned());
            c += 1;
        }
        // Pass default argument values
        for (key, value) in &self.default_values {
            if !kwargs.contains_key(key) {
                kwargs.insert(String::from(key), value.to_owned());
            }
        }
        // Check that all given arguments are taken by the command and
        // of the required type
        for (key, value) in &kwargs.clone() {
            if !self.accepted_kwargs.contains_key(key) {
                Err(ASCmdError {
                    command: String::from(&self.name),
                    details: CommandErrors::UndefinedArgument {
                        argument_name: String::from(key),
                        argument_type: value.get_type(),
                    },
                })?;
            }
            let arg_type = value.get_type();
            if !(self.accepted_kwargs[key] == ASType::Any && arg_type != ASType::VarRef)
                && self.accepted_kwargs[key] != arg_type
            {
                if arg_type == ASType::VarRef {
                    kwargs.insert(key.to_string(), info.get_var(value)?.clone());
                } else {
                    Err(ASCmdError {
                        command: String::from(&self.name),
                        details: CommandErrors::ArgumentTypeError {
                            argument_name: String::from(key),
                            required_type: self.accepted_kwargs[key].clone(),
                            given_type: value.get_type(),
                        },
                    })?;
                }
            }
        }
        // Check that all arguments in the command have a value
        for (key, value) in &self.accepted_kwargs {
            if !kwargs.contains_key(key) {
                Err(ASCmdError {
                    command: String::from(&self.name),
                    details: CommandErrors::MissingRequiredArgument {
                        argument_name: String::from(key),
                        argument_type: value.clone(),
                    },
                })?;
            }
        }
        (self.func)(info, kwargs)
    }
}

// Test command: shows all kwargs and their values

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
        func: |_info, kwargs| {
            for (key, arg) in kwargs {
                println!("{}: {:?}", key, arg);
            }
            Ok(())
        },
        args_to_kwargs: Vec::<String>::from([String::from("arg1"), String::from("arg2")]),
        accepted_kwargs: accepted,
        default_values: default,
    }
}

//TODO: *please* make this a macro
pub fn main_commands() -> HashMap<String, Command> {
    HashMap::<String, Command>::from([
        (
            "wait".to_string(),
            Command {
                name: String::from("wait"),
                func: |info, _kwargs| info.io().wait(),
                args_to_kwargs: Vec::<String>::new(),
                accepted_kwargs: HashMap::<String, ASType>::new(),
                default_values: HashMap::<String, ASVariable>::new(),
            },
        ),
        (
            "choice".to_string(),
            Command {
                name: String::from("choice"),
                func: |info, kwargs| {
                    let mut c = 1;
                    let mut choices = Vec::<&str>::new();
                    let mut gotos = Vec::<ASVariable>::new();
                    //get lists of the choices and gotos
                    while c <= 9 {
                        if c == 3 {
                            break;
                        } //Remove after proper choice command
                        let choice: &str = match &kwargs[&format!("ch{}", c)] {
                            ASVariable::String(c) => c,
                            _ => "",
                        };
                        let goto = &kwargs[&format!("go{}", c)];
                        //TODO: implement None default values
                        // if goto == ASVariable::None {
                        //     break;
                        // }
                        choices.append(&mut Vec::<&str>::from([choice]));
                        gotos.append(&mut Vec::<ASVariable>::from([goto.clone()]));
                        c += 1;
                    }
                    //get text
                    let text = match &kwargs["text"] {
                        ASVariable::String(c) => c,
                        _ => "",
                    };
                    //run io func and manage result
                    let pick = info.query(text, choices, true)?; //TODO: add allow_save
                    if pick == 0 {
                        // used in save/return/quit
                        info.pointer -= 1;
                        return Ok(());
                    };
                    info.goto_label(gotos.get((pick - 1) as usize).unwrap())?;
                    Ok(())
                },
                args_to_kwargs: Vec::<String>::from([String::from("text")]),
                accepted_kwargs: HashMap::<String, ASType>::from_iter([
                    (String::from("text"), ASType::String),
                    (String::from("ch1"), ASType::String),
                    (String::from("ch2"), ASType::String),
                    (String::from("ch3"), ASType::String),
                    (String::from("ch4"), ASType::String),
                    (String::from("ch5"), ASType::String),
                    (String::from("ch6"), ASType::String),
                    (String::from("ch7"), ASType::String),
                    (String::from("ch8"), ASType::String),
                    (String::from("ch9"), ASType::String),
                    (String::from("go1"), ASType::Label),
                    (String::from("go2"), ASType::Label),
                    (String::from("go3"), ASType::Label),
                    (String::from("go4"), ASType::Label),
                    (String::from("go5"), ASType::Label),
                    (String::from("go6"), ASType::Label),
                    (String::from("go7"), ASType::Label),
                    (String::from("go8"), ASType::Label),
                    (String::from("go9"), ASType::Label),
                ]),
                default_values: HashMap::<String, ASVariable>::from_iter([
                    (String::from("text"), ASVariable::String(String::from(""))),
                    (
                        String::from("ch1"),
                        ASVariable::String(String::from("Choice 1")),
                    ),
                    (
                        String::from("ch2"),
                        ASVariable::String(String::from("Choice 2")),
                    ),
                    (
                        String::from("ch3"),
                        ASVariable::String(String::from("Choice 3")),
                    ),
                    (
                        String::from("ch4"),
                        ASVariable::String(String::from("Choice 4")),
                    ),
                    (
                        String::from("ch5"),
                        ASVariable::String(String::from("Choice 5")),
                    ),
                    (
                        String::from("ch6"),
                        ASVariable::String(String::from("Choice 6")),
                    ),
                    (
                        String::from("ch7"),
                        ASVariable::String(String::from("Choice 7")),
                    ),
                    (
                        String::from("ch8"),
                        ASVariable::String(String::from("Choice 8")),
                    ),
                    (
                        String::from("ch9"),
                        ASVariable::String(String::from("Choice 9")),
                    ),
                    (String::from("go2"), ASVariable::Label(None)),
                    (String::from("go3"), ASVariable::Label(None)),
                    (String::from("go4"), ASVariable::Label(None)),
                    (String::from("go5"), ASVariable::Label(None)),
                    (String::from("go6"), ASVariable::Label(None)),
                    (String::from("go7"), ASVariable::Label(None)),
                    (String::from("go8"), ASVariable::Label(None)),
                    (String::from("go9"), ASVariable::Label(None)),
                ]),
            },
        ),
        (
            "goto".to_string(),
            Command {
                name: String::from("goto"),
                func: |info, kwargs| {
                    info.goto_label(&kwargs["pos"])?;
                    Ok(())
                },
                args_to_kwargs: Vec::<String>::from([String::from("pos")]),
                accepted_kwargs: HashMap::<String, ASType>::from_iter([(
                    String::from("pos"),
                    ASType::Label,
                )]),
                default_values: HashMap::<String, ASVariable>::new(),
            },
        ),
        (
            "ending".to_string(),
            Command {
                name: String::from("ending"),
                func: |info, kwargs| {
                    let name = match &kwargs["name"] {
                        ASVariable::String(c) => c,
                        _ => "",
                    };
                    info.io().show(&format!("Ending: {}", name))?;
                    info.quit();
                    Ok(())
                },
                args_to_kwargs: Vec::<String>::from([String::from("name")]),
                accepted_kwargs: HashMap::<String, ASType>::from_iter([(
                    String::from("name"),
                    ASType::String,
                )]),
                default_values: HashMap::<String, ASVariable>::from_iter([(
                    String::from("name"),
                    ASVariable::String(String::from("")),
                )]),
            },
        ),
        (
            "flag".to_string(),
            Command {
                name: "flag".to_string(),
                func: |info, kwargs| {
                    let flag = match kwargs.get("flag").unwrap() {
                        //Make sure you're getting a flag, not a variable
                        ASVariable::VarRef { name, .. } => ASVariable::VarRef {
                            name: name.to_string(),
                            flag: true,
                        },
                        _ => panic!(""),
                    };
                    info.set_var(&flag, kwargs.get("value").unwrap().clone())
                },
                accepted_kwargs: HashMap::<String, ASType>::from_iter([
                    (String::from("flag"), ASType::VarRef),
                    (String::from("value"), ASType::Bool),
                ]),
                default_values: HashMap::<String, ASVariable>::from_iter([(
                    String::from("value"),
                    ASVariable::Bool(true),
                )]),
                args_to_kwargs: vec![String::from("flag"), String::from("value")],
            },
        ),
        (
            "set".to_string(),
            Command {
                name: "set".to_string(),
                func: |info, kwargs| {
                    info.set_var(
                        kwargs.get("var").unwrap(),
                        kwargs.get("value").unwrap().clone(),
                    )
                },
                accepted_kwargs: HashMap::<String, ASType>::from_iter([
                    (String::from("var"), ASType::VarRef),
                    (String::from("value"), ASType::Any),
                ]),
                default_values: HashMap::<String, ASVariable>::new(),
                args_to_kwargs: vec![String::from("var"), String::from("value")],
            },
        ),
        (
            "add".to_string(),
            Command {
                name: "add".to_string(),
                func: |info, kwargs| {
                    let var = kwargs.get("var").unwrap();
                    let val = info.get_var(var)?.clone();
                    info.set_var(var, (val + kwargs.get("value").unwrap().clone())?)
                },
                accepted_kwargs: HashMap::<String, ASType>::from_iter([
                    (String::from("var"), ASType::VarRef),
                    (String::from("value"), ASType::Any),
                ]),
                default_values: HashMap::<String, ASVariable>::new(),
                args_to_kwargs: vec![String::from("var"), String::from("value")],
            },
        ),
        (
            "loadscript".to_string(),
            Command {
                name: "loadscript".to_string(),
                func: |info, kwargs| {
                    let script_name: &str = match kwargs.get("name").unwrap() {
                        ASVariable::String(c) => c,
                        _ => panic!(),
                    };
                    info.load_script(Some(script_name))
                },
                accepted_kwargs: HashMap::<String, ASType>::from_iter([(
                    String::from("name"),
                    ASType::String,
                )]),
                default_values: HashMap::<String, ASVariable>::new(),
                args_to_kwargs: vec![String::from("name")],
            },
        ),
        (
            "if".to_string(),
            Command {
                name: "if".to_string(),
                func: |info, kwargs| {
                    let condition = match kwargs.get("condition").unwrap() {
                        ASVariable::Bool(c) => *c,
                        _ => panic!(),
                    };
                    if condition {
                        info.goto_label(kwargs.get("gotrue").unwrap())
                    } else {
                        info.goto_label(kwargs.get("gofalse").unwrap())
                    }
                },
                accepted_kwargs: HashMap::<String, ASType>::from_iter([
                    (String::from("condition"), ASType::Bool),
                    (String::from("gotrue"), ASType::Label),
                    (String::from("gofalse"), ASType::Label),
                ]),
                default_values: HashMap::<String, ASVariable>::new(),
                args_to_kwargs: vec![
                    String::from("condition"),
                    String::from("gotrue"),
                    String::from("gofalse"),
                ],
            },
        ),
    ])
}
