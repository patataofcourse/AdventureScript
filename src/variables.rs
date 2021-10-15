use std::{cmp::PartialEq, collections::HashMap};

#[derive(Debug)]
pub enum ASType {
    Any,
    Bool,
    Int,
    String,
    List,
    Map,
}

impl PartialEq for ASType {
    fn eq(&self, other: &ASType) -> bool {
        if let Self::Any = other {
            return true;
        }
        match self {
            Self::Any => true,
            Self::Bool => {
                if let Self::Bool = other {
                    true
                } else {
                    false
                }
            }
            Self::Int => {
                if let Self::Int = other {
                    true
                } else {
                    false
                }
            }
            Self::String => {
                if let Self::String = other {
                    true
                } else {
                    false
                }
            }
            Self::List => {
                if let Self::List = other {
                    true
                } else {
                    false
                }
            }
            Self::Map => {
                if let Self::Map = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}

//TODO: add possibility for custom types???
#[derive(Debug)]
pub enum ASVariable {
    Bool(bool),
    Int(i32),
    String(String),
    List(Vec<ASVariable>),
    Map(HashMap<String, ASVariable>),
}

impl ASVariable {
    pub fn get_type(&self) -> ASType {
        match self {
            Self::Bool(_c) => ASType::Bool,
            Self::Int(_c) => ASType::Int,
            Self::String(_c) => ASType::String,
            Self::List(_c) => ASType::List,
            Self::Map(_c) => ASType::Map,
            //keep adding whenever you add more types
        }
    }
}
