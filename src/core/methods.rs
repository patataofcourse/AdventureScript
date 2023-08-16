use crate::{
    core::{
        error::{ASMethodError, ASSyntaxError, ASVarError, MethodErrors},
        ASType, ASVariable, GameInfo,
    },
    unwrap_var,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Method {
    pub name: String,
    func: fn(&GameInfo, &ASVariable, Vec<ASVariable>) -> anyhow::Result<ASVariable>,
    argtypes: Vec<ASType>,
    required_args: usize,
    deprecated: bool,
}

impl Method {
    pub fn run(
        self,
        info: &mut GameInfo,
        var: &ASVariable,
        mut args: Vec<ASVariable>,
    ) -> anyhow::Result<ASVariable> {
        if args.len() > self.argtypes.len() {
            Err(ASMethodError {
                method: String::from(&self.name),
                type_name: var.get_type().to_string(),
                details: MethodErrors::TooManyArguments {
                    given_args: args.len(),
                    max_args: self.argtypes.len(),
                },
            })?
        }

        let mut argnum = 0;
        for arg in &args.clone() {
            let arg_type = arg.get_type();
            if !(self.argtypes[argnum] == ASType::Any && arg_type != ASType::VarRef)
                && self.argtypes[argnum] != arg_type
            {
                if arg_type == ASType::VarRef {
                    args.insert(argnum, info.get_var(arg)?.clone());
                } else if arg_type == ASType::None && self.argtypes[argnum] == ASType::Label {
                    args.insert(argnum, ASVariable::Label(None));
                } else {
                    Err(ASMethodError {
                        method: String::from(&self.name),
                        type_name: var.get_type().to_string(),
                        details: MethodErrors::ArgumentTypeError {
                            argument_num: argnum,
                            required_type: self.argtypes[argnum].clone(),
                            given_type: arg.get_type(),
                        },
                    })?
                }
            }
            argnum += 1;
        }

        // Check that all required arguments in the method have been given
        if argnum < self.required_args {
            Err(ASMethodError {
                method: String::from(&self.name),
                type_name: var.get_type().to_string(),
                details: MethodErrors::MissingRequiredArgument {
                    argument_num: argnum,
                    argument_type: self.argtypes.get(argnum).unwrap().clone(),
                },
            })?;
        }

        if info.debug && self.deprecated {
            info.warn(format!(
                "Method '{}' for object type {} is deprecated",
                self.name,
                var.get_type()
            ));
        }

        (self.func)(info, var, args)
    }
}

#[derive(Clone)]
pub struct TypeMethods {
    methods: Vec<Method>,
    aliases: HashMap<String, String>,
}

impl TypeMethods {
    pub fn new() -> Self {
        Self {
            methods: vec![],
            aliases: HashMap::new(),
        }
    }
    pub fn basic() -> Self {
        Self::from(
            vec![Method {
                name: "str".to_string(),
                func: |info, var, _args| {
                    Ok(ASVariable::String(match var {
                        ASVariable::String(c) => format!("{:?}", c),
                        ASVariable::Object { spec, fields } => {
                            (info.get_object(spec).unwrap().stringify)(fields.clone())
                        }
                        var => var.to_string(),
                    }))
                },
                argtypes: vec![],
                required_args: 0,
                deprecated: false,
            }],
            HashMap::new(),
        )
    }
    pub fn from(methods: Vec<Method>, aliases: HashMap<String, String>) -> Self {
        Self { methods, aliases }
    }

    pub fn get(&self, name: &str) -> Option<&Method> {
        for method in &self.methods {
            if method.name == name {
                return Some(method);
            }
        }
        for (alias, a_name) in &self.aliases {
            if alias == name {
                return self.get(a_name);
            }
        }
        None
    }
    pub fn run_method(
        &self,
        name: &str,
        info: &mut GameInfo,
        var: &ASVariable,
        args: Vec<ASVariable>,
    ) -> anyhow::Result<ASVariable> {
        //TODO: Object/Module fields???
        match self.get(name) {
            Some(c) => c.clone().run(info, var, args),
            None => Err(ASSyntaxError::UnknownMethod(
                name.to_string(),
                var.get_type(),
            ))?,
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.methods.extend(other.methods);
        self.aliases.extend(other.aliases);
    }

    pub fn get_for_type(info: &GameInfo, type_: &ASType) -> Self {
        let mut out = match type_ {
            ASType::VarRef => todo!("Proper handling of VarRef methods"),
            ASType::List => Self::from(
                vec![
                    Method {
                        name: "get".to_string(),
                        func: |_info, var, args| -> anyhow::Result<ASVariable> {
                            let pos = *unwrap_var!(args -> 0; Int)?;
                            if pos < 0 {
                                Err(ASVarError::NegativeListIndex)?;
                            }
                            if let ASVariable::List(list) = var {
                                match list.get(pos as usize) {
                                    Some(c) => Ok(c.clone()),
                                    None => Err(ASVarError::WrongListIndex {
                                        num_items: list.len(),
                                        index: pos,
                                    })?,
                                }
                            } else {
                                panic!()
                            }
                        },
                        argtypes: vec![ASType::Int],
                        required_args: 1,
                        deprecated: false,
                    },
                    Method {
                        name: "index_of".to_string(),
                        func: |_info, var, args| -> anyhow::Result<ASVariable> {
                            if let ASVariable::List(list) = var {
                                let var = &args[0];
                                Ok(match list.iter().position(|r| r == var) {
                                    Some(c) => ASVariable::Int(c as i64),
                                    None => ASVariable::None,
                                })
                            } else {
                                panic!()
                            }
                        },
                        argtypes: vec![ASType::Any],
                        required_args: 1,
                        deprecated: false,
                    },
                ],
                HashMap::new(),
            ),
            ASType::Map => Self::from(
                vec![Method {
                    name: "get".to_string(),
                    func: |_info, var, args| -> anyhow::Result<ASVariable> {
                        let key = match args.get(0) {
                            Some(c) => c.clone(),
                            None => panic!(),
                        }
                        .clone()
                        .as_key()?;

                        if let ASVariable::Map(map) = var {
                            match map.get(&key) {
                                Some(c) => Ok(c.clone()),
                                None => Err(ASVarError::WrongMapKey { key })?,
                            }
                        } else {
                            panic!()
                        }
                    },
                    argtypes: vec![ASType::Any],
                    required_args: 1,
                    deprecated: false,
                }],
                HashMap::new(),
            ),
            ASType::Object(spec) => info.get_object(spec).unwrap().methods,
            _ => Self::new(),
        };
        out.extend(Self::basic());
        out
    }
}

impl Default for TypeMethods {
    fn default() -> Self {
        Self::new()
    }
}
