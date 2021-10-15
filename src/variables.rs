use std::collections::HashMap;

#[derive(Debug)]
pub enum ASType {
    Any,
    Bool,
    Int,
    String,
    List,
    Map,
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
