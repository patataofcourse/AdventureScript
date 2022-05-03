use crate::{
    command,
    core::{
        error::{ASCmdError, ASGameError, CommandErrors},
        ASType, ASVariable, GameInfo,
    },
    formats::save,
    unwrap_var,
};
use anyhow;
use std::{collections::HashMap, iter::FromIterator};

#[derive(Clone)]
pub struct Command {
    pub name: String,
    func: fn(&mut GameInfo, HashMap<String, ASVariable>) -> anyhow::Result<()>,
    args_to_kwargs: Vec<String>,
    accepted_kwargs: HashMap<String, ASType>,
    default_values: HashMap<String, ASVariable>,
    pub deprecated: bool,
}

#[derive(Clone)]
pub struct CmdSet {
    pub commands: Vec<Command>,
    pub aliases: HashMap<String, String>,
    pub modules: HashMap<String, CmdSet>, //TODO: make it work
}

impl CmdSet {
    pub fn get(&self, name: &str) -> Option<&Command> {
        let (module, name) = name.split_once(".").unwrap_or(("", name));
        let module = if module != "" {
            self.modules.get(module)?
        } else {
            self
        };
        for command in &module.commands {
            if command.name == name {
                return Some(command);
            }
        }
        for (alias, a_name) in &module.aliases {
            if alias == name {
                return module.get(a_name);
            }
        }
        None
    }
    pub fn extend(&mut self, other: Self) {
        self.commands.extend(other.commands);
        self.aliases.extend(other.aliases);
    }
    pub fn from(commands: Vec<Command>, aliases: HashMap<String, String>) -> Self {
        Self {
            commands,
            aliases,
            modules: HashMap::new(),
        }
    }
    pub fn new() -> Self {
        Self {
            commands: vec![],
            aliases: HashMap::new(),
            modules: HashMap::new(),
        }
    }
    pub(crate) fn add_module(&mut self, commands: CmdSet, name: &str) {
        self.modules.insert(name.to_string(), commands);
    }
}

impl Command {
    pub fn new(
        name: String,
        func: fn(&mut GameInfo, HashMap<String, ASVariable>) -> anyhow::Result<()>,
        args_to_kwargs: Vec<String>,
        accepted_kwargs: HashMap<String, ASType>,
        default_values: HashMap<String, ASVariable>,
        deprecated: bool,
    ) -> Self {
        //TODO: disallow None and Empty type arguments
        //TODO: make sure arg ordering/defaults are well done
        Self {
            name,
            func,
            args_to_kwargs,
            accepted_kwargs,
            default_values,
            deprecated,
        }
    }
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
                        max_args: self.args_to_kwargs.len(),
                        given_args: args.len(),
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
                } else if arg_type == ASType::None && self.accepted_kwargs[key] == ASType::Label {
                    kwargs.insert(key.to_string(), ASVariable::Label(None));
                } else {
                    if self.args_to_kwargs.contains(&String::from(key)) {
                        Err(ASCmdError {
                            command: String::from(&self.name),
                            details: CommandErrors::PosArgTypeError {
                                argument_name: String::from(key),
                                argument_num: self
                                    .args_to_kwargs
                                    .iter()
                                    .position(|r| r == &String::from(key))
                                    .unwrap(),
                                required_type: self.accepted_kwargs[key].clone(),
                                given_type: value.get_type(),
                            },
                        })
                    } else {
                        Err(ASCmdError {
                            command: String::from(&self.name),
                            details: CommandErrors::ArgumentTypeError {
                                argument_name: String::from(key),
                                required_type: self.accepted_kwargs[key].clone(),
                                given_type: value.get_type(),
                            },
                        })?
                    }?;
                }
            }
        }
        // Check that all arguments in the command have been given
        for (key, value) in &self.accepted_kwargs {
            if !kwargs.contains_key(key) {
                if self.args_to_kwargs.contains(&String::from(key)) {
                    Err(ASCmdError {
                        command: String::from(&self.name),
                        details: CommandErrors::MissingRequiredPosArg {
                            argument_name: String::from(key),
                            argument_num: self
                                .args_to_kwargs
                                .iter()
                                .position(|r| r == &String::from(key))
                                .unwrap(),
                            argument_type: value.clone(),
                        },
                    })
                } else {
                    Err(ASCmdError {
                        command: String::from(&self.name),
                        details: CommandErrors::MissingRequiredArgument {
                            argument_name: String::from(key),
                            argument_type: value.clone(),
                        },
                    })?
                }?;
            }
        }

        if info.debug && self.deprecated {
            info.warn(format!("Command '{}' is deprecated", self.name));
        }

        (self.func)(info, kwargs)
    }
}

pub fn main_commands() -> CmdSet {
    CmdSet::from(
        vec![
            command! {
                wait => |info, _kwargs| {
                    info.wait()
                }
            },
            command! {
                choice (
                    !text: String,
                    !choice1: List,
                    choice2: List = vec![],
                    choice3: List = vec![],
                    choice4: List = vec![],
                    choice5: List = vec![],
                    choice6: List = vec![],
                    choice7: List = vec![],
                    choice8: List = vec![],
                    choice9: List = vec![],
                ) => |info, kwargs| {
                    let mut c = 1;
                    let mut texts = Vec::<String>::new();
                    let mut gotos = Vec::<ASVariable>::new();
                    // separate the choices into the vectors defined above
                    while c <= 9 {
                        let choice = unwrap_var!(kwargs -> &format!("choice{}", c); List)?;
                        let text = match choice.get(0) {
                            Some(s) => match s {
                                ASVariable::String(c) => c.to_string(),
                                ASVariable::VarRef {name, flag} => {
                                    match info.get_var(&ASVariable::VarRef {name: name.to_string(), flag: *flag})? {
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
                                    match info.get_var(&ASVariable::VarRef {name: name.to_string(),flag: *flag})? {
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
                    let text = unwrap_var!(kwargs -> "text"; String)?;
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
                goto (!pos: Label, ) => |info, kwargs| {
                    info.goto_label(&kwargs["pos"])
                }
            },
            command! {
                ending (name: String = "".to_string(), ) => |info, kwargs| {
                    let name = unwrap_var!(kwargs -> "name"; String)?;
                    info.show(&format!("Ending: {}", name))?;
                    info.quit();
                    Ok(())
                }
            },
            command! {
                flag (!flag: VarRef, value: Bool = true, ) => |info, kwargs| {
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
              set (!var: VarRef, !value: Any,) => |info, kwargs| {
                    let mut val = kwargs.get("value").unwrap().clone();
                    while val.get_type() == ASType::VarRef {
                        val = info.get_var(&val)?.clone();
                    }
                    info.set_var(
                        kwargs.get("var").unwrap(),
                        val,
                    )
                }
            },
            command! {
                add (!var: VarRef, !value: Any,) => |info, kwargs| {
                    let var = kwargs.get("var").unwrap();
                    let val = info.get_var(var)?.clone();
                    info.set_var(var, (val + kwargs.get("value").unwrap().clone())?)
                }
            },
            command! {
                loadscript (!name: String,) => |info, kwargs| {
                    let script_name: &str = unwrap_var!(kwargs -> "name"; String)?;
                    info.load_script(Some(script_name))
                }
            },
            command! {
                if (!condition: Bool, !gotrue: Label, !gofalse: Label, ) => |info, kwargs| {
                    let condition = *unwrap_var!(kwargs -> "condition"; Bool)?;
                    if condition {
                        info.goto_label(kwargs.get("gotrue").unwrap())
                    } else {
                        info.goto_label(kwargs.get("gofalse").unwrap())
                    }
                }
            },
            command! {
                error (!message: String, ) => |_info, kwargs| {
                    let message = unwrap_var!(kwargs -> "message"; String)?.to_string();
                    Err(ASGameError(message))?
                }
            },
            command! {
                save (!val: Bool, ) => |info, kwargs| {
                    info.allow_save = *unwrap_var!(kwargs -> "val"; Bool)?;
                    Ok(())
                }
            },
            command! {
                gameover => |info, _kwargs| {
                    info.show("**GAME OVER**")?;
                    let query = info.query("Start over from last save?", vec!("Yes","No"))?;
                    if query == 1 {
                        if !save::restore(info)? {
                            info.quit();
                        };
                    } else {
                        info.quit();
                    }
                    Ok(())
                }
            },
            command! {
                del (!var: VarRef,) => |info, kwargs| {
                    info.del_var(kwargs.get("var").unwrap())
                }
            },
            command! {
                switch (
                    !check: Any,
                    !values: List,
                    !gotos: List,
                    default: Label = None,
                ) => |info, kwargs| {
                    let check = kwargs.get("check").unwrap();
                    let values = unwrap_var!(kwargs -> "values"; List)?;
                    let labels = unwrap_var!(kwargs -> "gotos"; List)?;
                    let default = kwargs.get("default").unwrap();

                    if values.len() != labels.len() {
                        Err(ASCmdError {
                            command: "switch".to_string(),
                            details: CommandErrors::SwitchParams(values.len(), labels.len()),
                        })?
                    }

                    let mut c = 0; // counter
                    for value in values {
                        let mut value = value.clone();
                        while value.get_type() == ASType::VarRef {
                            value = info.get_var(&value)?.clone();
                        }

                        if &value == check {
                            let label = labels.get(c).unwrap();
                            if label.get_type() != ASType::Label {
                                Err(ASCmdError {
                                    command: "switch".to_string(),
                                    details: CommandErrors::SwitchLabelType{
                                        number: c,
                                        given: label.get_type(),
                                    }
                                })?
                            }

                            info.goto_label(label)?;
                            return Ok(())
                        }
                        c += 1;
                    }
                    info.goto_label(default)
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
            ("lose".to_string(), "gameover".to_string()),
        ]),
    )
}
