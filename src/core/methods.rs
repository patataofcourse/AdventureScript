use crate::{
    core::{error::ASVarError, ASType, ASVariable, GameInfo},
    unwrap_var,
};

#[derive(Clone)]
pub struct Method {
    pub name: String,
    func: fn(&GameInfo, &ASVariable, Vec<&ASVariable>) -> anyhow::Result<ASVariable>,
    //TODO: potentially check args?
}

impl Method {
    pub fn run(
        self,
        info: &GameInfo,
        var: &ASVariable,
        args: Vec<&ASVariable>,
    ) -> anyhow::Result<ASVariable> {
        //TODO: arg checking
        (self.func)(info, var, args)
    }
}

#[derive(Clone)]
pub struct TypeMethods {
    methods: Vec<Method>,
}

impl TypeMethods {
    pub fn new() -> Self {
        Self { methods: vec![] }
    }
    pub fn basic() -> Self {
        Self::from(vec![Method {
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
        }])
    }
    pub fn from(methods: Vec<Method>) -> Self {
        Self { methods }
    }

    pub fn get(&self, name: &str) -> Option<&Method> {
        for method in &self.methods {
            if method.name == name {
                return Some(method);
            }
        }
        //TODO: aliases?
        /*
        for (alias, a_name) in &self.aliases {
            if alias == name {
                return self.get(a_name);
            }
        }
        */
        None
    }
    pub fn extend(&mut self, other: Self) {
        self.methods.extend(other.methods);
        //self.aliases.extend(other.aliases);
    }

    pub fn get_for_type(info: &GameInfo, type_: &ASType) -> Self {
        let mut out = match type_ {
            ASType::List => Self::from(vec![Method {
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
            }]),
            ASType::Map => Self::from(vec![Method {
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
                            None => Err(ASVarError::WrongMapKey { key: key })?,
                        }
                    } else {
                        panic!()
                    }
                },
            }]),
            ASType::Object(spec) => info.get_object(spec).unwrap().methods,
            _ => Self::new(),
        };
        out.extend(Self::basic());
        out
    }
}
