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
    Int(i32),
    Bool(bool),
    String(String),
    List(Vec<ASVariable>),
    Map(HashMap<String, ASVariable>),
}
