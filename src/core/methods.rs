use crate::{
    core::{error::ASVarError, ASType, ASVariable},
    unwrap_var,
};

pub struct Method {
    pub name: String,
    func: fn(ASVariable, Vec<ASVariable>) -> anyhow::Result<ASVariable>,
    //TODO: potentially check args?
}

impl Method {
    pub fn run(self, var: ASVariable, args: Vec<ASVariable>) -> anyhow::Result<ASVariable> {
        //TODO: arg checking
        (self.func)(var, args)
    }
}

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
            func: |var, _args| {
                Ok(ASVariable::String(match var {
                    ASVariable::String(c) => format!("{:?}", c),
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

    pub fn get_for_type(type_: ASType) -> Self {
        let mut out = match type_ {
            ASType::List => Self::from(vec![Method {
                name: "get".to_string(),
                func: |var, args: Vec<ASVariable>| -> anyhow::Result<ASVariable> {
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
                func: |var, args: Vec<ASVariable>| -> anyhow::Result<ASVariable> {
                    let key = match args.get(0) {
                        Some(c) => c.clone(),
                        None => panic!(),
                    }
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
            _ => Self::new(),
        };
        out.extend(Self::basic());
        out
    }
}
