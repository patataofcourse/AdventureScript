use crate::core::{
    error::{ASCmdError, CommandErrors},
    ASType, ASVariable, GameInfo,
};
use std::collections::HashMap;

mod definitions;

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
        let (module, name) = name.split_once('.').unwrap_or(("", name));
        let module = if !module.is_empty() {
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

impl Default for CmdSet {
    fn default() -> Self {
        Self::new()
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
        let mut kwargs = kwargs;
        // Turn positional arguments into keyword arguments
        for (c, arg) in args.iter().enumerate() {
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

pub use definitions::main_commands;
