use crate::{
    command,
    core::{
        error::{ASCmdError, ASGameError, CommandErrors},
        info::GameInfo,
        variables::{ASType, ASVariable},
    },
    get_var,
};
use anyhow;
use std::{collections::HashMap, iter::FromIterator};

pub struct Command {
    pub name: String,
    func: fn(&Self, &mut GameInfo, HashMap<String, ASVariable>) -> anyhow::Result<()>,
    args_to_kwargs: Vec<String>,
    accepted_kwargs: HashMap<String, ASType>,
    default_values: HashMap<String, ASVariable>,
}

pub struct CmdSet {
    pub commands: Vec<Command>,
    pub aliases: HashMap<String, String>,
}

impl CmdSet {
    pub fn get(&self, name: &str) -> Option<&Command> {
        let mut out = None;
        for command in &self.commands {
            if command.name == name {
                out = Some(command);
                break;
            }
        }
        for (alias, a_name) in &self.aliases {
            if alias == name {
                out = self.get(a_name);
                break;
            }
        }
        out
    }
    pub fn extend(&mut self, other: Self) {
        //TODO: adapt for modules
        self.commands.extend(other.commands);
        self.aliases.extend(other.aliases);
    }
    pub fn from(commands: Vec<Command>, aliases: HashMap<String, String>) -> Self {
        Self { commands, aliases }
    }
    pub fn new() -> Self {
        Self {
            commands: vec![],
            aliases: HashMap::new(),
        }
    }
}

impl Command {
    pub fn run(
        &self,
        info: &mut GameInfo,
        args: Vec<ASVariable>,
        kwargs: HashMap<String, ASVariable>,
    ) -> anyhow::Result<()> {
        //TODO: disallow None type arguments
        //TODO: implement or_none
        //TODO: make sure arg ordering/defaults are well done

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
        (self.func)(&self, info, kwargs)
    }
}

pub fn main_commands() -> CmdSet {
    CmdSet::from(
        vec![
            command! {
                "wait" => |_cmd, info, _kwargs| {
                    info.io().wait()
                }
            },
            command! {
                "choice" (
                    !"text": String,
                    !"choice1": List,
                    "choice2": List = vec![],
                    "choice3": List = vec![],
                    "choice4": List = vec![],
                    "choice5": List = vec![],
                    "choice6": List = vec![],
                    "choice7": List = vec![],
                    "choice8": List = vec![],
                    "choice9": List = vec![],
                ) => |_cmd, info, kwargs| {
                    let mut c = 1;
                    let mut texts = Vec::<String>::new();
                    let mut gotos = Vec::<ASVariable>::new();
                    // separate the choices into the vectors defined above
                    while c <= 9 {
                        let choice = get_var!(kwargs -> &format!("choice{}", c); List);
                        let text = match choice.get(0) {
                            Some(s) => match s {
                                ASVariable::String(c) => c.to_string(),
                                ASVariable::VarRef {name, flag} => {
                                    match info.get_var(&ASVariable::VarRef {name: name.to_string(),flag: *flag,})? {
                                            ASVariable::String(c) => c.to_string(),
                                            other => Err(ASCmdError {
                                            command: "choice".to_string(),
                                            details: CommandErrors::ChoiceWrongType{
                                                choice: c,
                                                number: 2,
                                                given: other.get_type(),
                                                asked: ASType::Bool
                                            }
                                        })?,
                                    }
                                },
                                other => Err(ASCmdError {
                                    command: "choice".to_string(),
                                    details: CommandErrors::ChoiceWrongType{
                                        choice: c,
                                        number: 0,
                                        given: other.get_type(),
                                        asked: ASType::String
                                    }
                                })?,
                            },
                            None => break
                        };
                        let goto = match choice.get(1){
                            Some(v) => match v {
                                ASVariable::None => ASVariable::Label(None),
                                ASVariable::Label{..} => v.clone(),
                                _ => Err(ASCmdError {
                                    command: "choice".to_string(),
                                    details: CommandErrors::ChoiceWrongType{
                                        choice: c,
                                        number: 1,
                                        given: v.get_type(),
                                        asked: ASType::Label
                                    }
                                })?
                            },
                            None => Err(ASCmdError {
                                command: "choice".to_string(),
                                details: CommandErrors::ChoiceMissingRequired{typ: ASType::Label, choice: c},
                            })?,
                        };
                        let flag = match choice.get(2) {
                            Some(l) => match l {
                                ASVariable::Bool(c) => *c,
                                ASVariable::VarRef {name, flag} => {
                                    match info.get_var(&ASVariable::VarRef {name: name.to_string(),flag: *flag,})? {
                                            ASVariable::Bool(c) => *c,
                                            other => Err(ASCmdError {
                                            command: "choice".to_string(),
                                            details: CommandErrors::ChoiceWrongType{
                                                choice: c,
                                                number: 2,
                                                given: other.get_type(),
                                                asked: ASType::Bool
                                            }
                                        })?,
                                    }
                                },
                                other => Err(ASCmdError {
                                    command: "choice".to_string(),
                                    details: CommandErrors::ChoiceWrongType{
                                        choice: c,
                                        number: 2,
                                        given: other.get_type(),
                                        asked: ASType::Bool
                                    }
                                })?,
                            },
                            None => true,
                        };
                        if flag {
                            texts.push(text);
                            gotos.push(goto.clone());
                        }
                        c += 1
                    }
                    let mut text_refs: Vec<&str> = vec![];
                    for t in &texts {
                        text_refs.push(t);
                    }
                    let text = get_var!(kwargs -> "text"; String);
                    let pick = info.query(text, text_refs)?;
                    if pick == 0 {
                        // used in save/return/quit
                        info.pointer -= 1;
                        return Ok(());
                    };
                    info.goto_label(gotos.get((pick - 1) as usize).unwrap())?;
                    Ok(())
                }
            },
            command! {
                "goto" (!"pos": Label, ) => |_cmd, info, kwargs| {
                    info.goto_label(&kwargs["pos"])
                }
            },
            command! {
                "ending" ("name": String = "".to_string(), ) => |_cmd, info, kwargs| {
                    let name = get_var!(kwargs -> "name"; String);
                    info.io().show(&format!("Ending: {}", name))?;
                    info.quit();
                    Ok(())
                }
            },
            command! {
                "flag" (!"flag": VarRef, "value": Bool = true, ) => |_cmd, info, kwargs| {
                    let flag = match kwargs.get("flag").unwrap() {
                        //Make sure you're getting a flag, not a variable
                        ASVariable::VarRef { name, .. } => ASVariable::VarRef {
                            name: name.to_string(),
                            flag: true,
                        },
                        _ => panic!(),
                    };
                    info.set_var(&flag, kwargs.get("value").unwrap().clone())
                }
            },
            command! {
                "set" (!"var": VarRef, !"value": Any,) => |_cmd, info, kwargs| {
                    info.set_var(
                        kwargs.get("var").unwrap(),
                        kwargs.get("value").unwrap().clone(),
                    )
                }
            },
            command! {
                "add" (!"var": VarRef, !"value": Any,) => |_cmd, info, kwargs| {
                    let var = kwargs.get("var").unwrap();
                    let val = info.get_var(var)?.clone();
                    info.set_var(var, (val + kwargs.get("value").unwrap().clone())?)
                }
            },
            command! {
                "loadscript" (!"name": String,) => |_cmd, info, kwargs| {
                    let script_name: &str = get_var!(kwargs -> "name"; String);
                    info.load_script(Some(script_name))
                }
            },
            command! {
                "if" (!"condition": Bool, !"gotrue": Label, !"gofalse": Label, ) => |_cmd, info, kwargs| {
                    let condition = *get_var!(kwargs -> "condition"; Bool);
                    if condition {
                        info.goto_label(kwargs.get("gotrue").unwrap())
                    } else {
                        info.goto_label(kwargs.get("gofalse").unwrap())
                    }
                }
            },
            command! {
                "error" (!"message": String, ) => |_cmd, _info, kwargs| {
                    let message = get_var!(kwargs -> "message"; String).to_string();
                    Err(ASGameError(message))?
                }
            },
            command! {
                "save" (!"val": Bool, ) => |_cmd, info, kwargs| {
                    info.allow_save = *get_var!(kwargs -> "val"; Bool);
                    Ok(())
                }
            },
        ],
        HashMap::from_iter([
            ("w".to_string(), "wait".to_string()),
            ("sv".to_string(), "save".to_string()),
            ("go".to_string(), "goto".to_string()),
            ("ch".to_string(), "choice".to_string()),
            ("end".to_string(), "ending".to_string()),
            ("load".to_string(), "loadscript".to_string()),
            ("ld".to_string(), "loadscript".to_string()),
        ]),
    )
}
