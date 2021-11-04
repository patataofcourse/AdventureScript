use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ASType {
    Any,
    Bool,
    Int,
    String,
    List,
    Map,
    None,
}

impl Display for ASType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

//TODO: add possibility for custom types???
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ASVariable {
    Bool(bool),
    Int(i32),
    String(String),
    List(Vec<ASVariable>),
    Map(HashMap<String, ASVariable>),
    None,
}

impl ASVariable {
    pub fn get_type(&self) -> ASType {
        match self {
            Self::Bool(_c) => ASType::Bool,
            Self::Int(_c) => ASType::Int,
            Self::String(_c) => ASType::String,
            Self::List(_c) => ASType::List,
            Self::Map(_c) => ASType::Map,
            Self::None => ASType::None,
            //keep adding whenever you add more types
        }
    }
}
