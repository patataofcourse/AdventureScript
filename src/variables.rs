use super::error::{ASSyntaxError, SyntaxErrors};
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Sub},
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

impl Add for ASVariable {
    type Output = anyhow::Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            Self::Int(c) => {
                if let ASVariable::Int(c2) = rhs {
                    Ok(ASVariable::Int(c + c2))
                } else {
                    op_err("add".to_string(), self, rhs)
                }
            }
            Self::String(c) => {
                if let ASVariable::String(c2) = rhs {
                    Ok(ASVariable::String(c.to_string() + &c2))
                } else {
                    op_err("add".to_string(), self, rhs)
                }
            }
            //TODO: lists and maps
            _ => op_err("add".to_string(), self, rhs),
        }
    }
}

impl Sub for ASVariable {
    type Output = anyhow::Result<Self>;
    fn sub(self, rhs: Self) -> anyhow::Result<Self> {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = rhs {
                return Ok(ASVariable::Int(c - c2));
            }
        };
        op_err("sub".to_string(), self, rhs)
    }
}

impl Mul for ASVariable {
    type Output = anyhow::Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        match &self {
            Self::Int(c) => {
                if let ASVariable::Int(c2) = rhs {
                    Ok(ASVariable::Int(c + c2))
                } else {
                    op_err("add".to_string(), self, rhs)
                }
            }
            Self::String(c) => {
                if let ASVariable::Int(c2) = rhs {
                    let mut result = String::new();
                    for n in 0..c2 {
                        result += &c;
                    }
                    Ok(ASVariable::String(result))
                } else {
                    op_err("add".to_string(), self, rhs)
                }
            }
            //TODO: lists and maps
            _ => op_err("add".to_string(), self, rhs),
        }
    }
}

impl Div for ASVariable {
    type Output = anyhow::Result<Self>;
    fn div(self, rhs: Self) -> anyhow::Result<Self> {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = rhs {
                return Ok(ASVariable::Int(c / c2));
            }
        };
        op_err("div".to_string(), self, rhs)
    }
}

impl ASVariable {
    pub fn pow(self, exponent: Self) -> anyhow::Result<Self> {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = exponent {
                return Ok(ASVariable::Int(c.pow(c2 as u32)));
            }
        };
        op_err("div".to_string(), self, exponent)
    }
}

fn op_err(op: String, v1: ASVariable, v2: ASVariable) -> anyhow::Result<ASVariable> {
    Err(ASSyntaxError {
        details: SyntaxErrors::OperationNotDefined {
            op: "add".to_string(),
            type1: v1.get_type(),
            type2: v2.get_type(),
        },
    })?
}
