use super::error::ASSyntaxError;
use std::{
    cmp::{Ordering, PartialOrd},
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ASType {
    Any,
    Bool,
    Int,
    String,
    List,
    Map,
    VarRef,
    None,
}

impl Display for ASType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ASVariable {
    Bool(bool),
    Int(i32),
    String(String),
    List(Vec<ASVariable>),
    Map(HashMap<KeyVar, ASVariable>),
    VarRef { name: String, flag: bool },
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
            Self::VarRef { .. } => ASType::VarRef, // Variable name, for defining new variables
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
    fn sub(self, rhs: Self) -> Self::Output {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = rhs {
                return Ok(ASVariable::Int(c - c2));
            }
        };
        op_err("substract".to_string(), self, rhs)
    }
}

impl Mul for ASVariable {
    type Output = anyhow::Result<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        match &self {
            Self::Int(c) => {
                if let ASVariable::Int(c2) = rhs {
                    Ok(ASVariable::Int(c * c2))
                } else {
                    op_err("multiply".to_string(), self, rhs)
                }
            }
            Self::String(c) => {
                if let ASVariable::Int(c2) = rhs {
                    let mut result = String::new();
                    for _ in 0..c2 {
                        result += &c;
                    }
                    Ok(ASVariable::String(result))
                } else {
                    op_err("multiply".to_string(), self, rhs)
                }
            }
            //TODO: lists and maps
            _ => op_err("multiply".to_string(), self, rhs),
        }
    }
}

impl Div for ASVariable {
    type Output = anyhow::Result<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = rhs {
                return Ok(ASVariable::Int(c / c2));
            }
        };
        op_err("divide".to_string(), self, rhs)
    }
}

impl Neg for ASVariable {
    type Output = anyhow::Result<Self>;
    fn neg(self) -> Self::Output {
        match self {
            Self::Int(c) => Ok(Self::Int(-c)),
            _ => unary_op_err("negate".to_string(), self),
        }
    }
}

impl Not for ASVariable {
    type Output = anyhow::Result<Self>;
    fn not(self) -> Self::Output {
        match self {
            Self::Bool(c) => Ok(Self::Bool(!c)),
            _ => unary_op_err("bool-negate".to_string(), self),
        }
    }
}

impl ASVariable {
    pub fn pow(self, exponent: Self) -> anyhow::Result<Self> {
        if let Self::Int(c) = self {
            if let Self::Int(c2) = exponent {
                return Ok(ASVariable::Int(c.pow(c2 as u32)));
            }
        };
        op_err("calculate the power of".to_string(), self, exponent)
    }
}

impl PartialOrd for ASVariable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            ASVariable::Int(c) => {
                if let ASVariable::Int(d) = other {
                    c.partial_cmp(d)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn op_err(op: String, v1: ASVariable, v2: ASVariable) -> anyhow::Result<ASVariable> {
    Err(ASSyntaxError::OperationNotDefined {
        op: op,
        type1: v1.get_type(),
        type2: v2.get_type(),
    })?
}

fn unary_op_err(op: String, v: ASVariable) -> anyhow::Result<ASVariable> {
    Err(ASSyntaxError::UnaryOperationNotDefined {
        op: op,
        type1: v.get_type(),
    })?
}

impl Display for ASVariable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None".to_string(),
                Self::Bool(c) => if *c { "true" } else { "false" }.to_string(),
                Self::Int(c) => c.to_string(),
                Self::String(c) => format!("\"{}\"", c),
                Self::List(c) => format!("[{}]", {
                    let mut out = String::new();
                    let mut first_elem = true;
                    for element in c {
                        if first_elem {
                            first_elem = false;
                        } else {
                            out += ", ";
                        }
                        out += &element.to_string();
                    }
                    out
                }),
                Self::Map(c) => format!("{{{}}}", {
                    let mut out = String::new();
                    let mut first_elem = true;
                    for (key, value) in c {
                        if first_elem {
                            first_elem = false;
                        } else {
                            out += ", "
                        }
                        out += &format!("{}: {}", key, value);
                    }
                    out
                }),
                Self::VarRef { name, flag } => {
                    format!("{} {}", if *flag { "Flag" } else { "Variable" }, name)
                }
            }
        )
    }
}

// Struct for ASVariable values that can be used as keys in a Map
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum KeyVar {
    Bool(bool),
    Int(i32),
    String(String),
    None,
}

impl KeyVar {
    pub fn new(val: ASVariable) -> anyhow::Result<Self> {
        Ok(match val {
            ASVariable::None => Self::None,
            ASVariable::Bool(c) => Self::Bool(c),
            ASVariable::Int(c) => Self::Int(c),
            ASVariable::String(c) => Self::String(c),
            _ => Err(ASSyntaxError::InvalidMapKey {
                key_type: val.get_type(),
            })?,
        })
    }
    pub fn get(&self) -> ASVariable {
        match self {
            Self::None => ASVariable::None,
            Self::Int(c) => ASVariable::Int(*c),
            Self::String(c) => ASVariable::String(c.to_string()),
            Self::Bool(c) => ASVariable::Bool(*c),
        }
    }
}

impl ASVariable {
    pub fn as_key(self) -> anyhow::Result<KeyVar> {
        KeyVar::new(self)
    }
}

impl Display for KeyVar {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.get())
    }
}
