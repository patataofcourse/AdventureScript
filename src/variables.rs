use std::collections::HashMap;

pub enum ASType {
    Int,
    String,
    List,
    Map,
}

//TODO: add possibility for custom types???
pub enum ASVariable {
    Int(i32),
    String(String),
    List(Vec<ASVariable>),
    Map(HashMap<String, ASVariable>),
}
