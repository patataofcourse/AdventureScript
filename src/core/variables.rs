use crate::core::error::ASSyntaxError;
use serde_derive::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, PartialOrd},
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

/// Enum listing all possible types for AdventureScript variables. To see what each type means, check
/// the `ASVariable` documentation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ASType {
    /// Not properly a type, but meant to be used when there's compatibility with any type (for example,
    /// in a command argument that may be of any type).
    Any,
    Bool,
    Int,
    String,
    List,
    Map,
    Label,
    VarRef,
    None,
    Object(String),
}

impl Display for ASType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

/// Enum used to handle AdventureScript variables.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ASVariable {
    /// Boolean value (true/false)
    Bool(bool),
    /// Integer value (32-bit signed)
    Int(i32),
    /// String value
    String(String),
    /// List value (vector of other `ASVariable`s of any type)
    List(Vec<ASVariable>),
    /// Map value (uses `HashMap`, keys can only be of a type that exists in the `KeyVar` enum)
    Map(HashMap<KeyVar, ASVariable>),
    /// A reference to a label, by name. If `None`, don't jump anywhere, instead continue to the next
    /// line as usual.
    Label(Option<String>),
    /// A reference to a variable or flag. Meant to be used with commands that define a variable or
    /// change its content.
    VarRef {
        /// The name of the variable or flag.
        name: String,
        /// Whether it's a variable or a flag.
        flag: bool,
    },
    /// Empty value, equivalent to Rust's `()`.
    None,
    /// A custom object/"class" type, to be used in modules
    Object {
        name: String,
        fields: HashMap<String, ASVariable>,
        //TODO: add spec for object specification (available in info)
    },
}

impl ASVariable {
    /// Get an `ASType` representing the variable's type.
    pub fn get_type(&self) -> ASType {
        match self {
            Self::Bool(_) => ASType::Bool,
            Self::Int(_) => ASType::Int,
            Self::String(_) => ASType::String,
            Self::List(_) => ASType::List,
            Self::Map(_) => ASType::Map,
            Self::Label(_) => ASType::Label,
            Self::VarRef { .. } => ASType::VarRef,
            Self::None => ASType::None,
            Self::Object { name, .. } => ASType::Object(name.clone()),
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
    /// Meant for use with `Int`-type variables, exponent/power function
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
                Self::String(c) => format!("{}", c),
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
                Self::Label(c) => match c {
                    Some(c) => format!("Label {{{}}}", c),
                    None => String::from("Null label"),
                },
                Self::VarRef { name, flag } => {
                    format!("{} {}", if *flag { "Flag" } else { "Variable" }, name)
                }
                Self::Object { name, .. } => {
                    //TODO: refer to spec instead
                    format!("<Object type {}>", name)
                }
            }
        )
    }
}

/// Struct for `ASVariable` values that can be used as keys in a `Map`.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum KeyVar {
    Bool(bool),
    Int(i32),
    String(String),
}

impl KeyVar {
    /// Creates a `KeyVar` from an `ASVariable` or returns an error if not compatible.
    pub fn new(val: ASVariable) -> anyhow::Result<Self> {
        Ok(match val {
            ASVariable::Bool(c) => Self::Bool(c),
            ASVariable::Int(c) => Self::Int(c),
            ASVariable::String(c) => Self::String(c),
            _ => Err(ASSyntaxError::InvalidMapKey {
                key_type: val.get_type(),
            })?,
        })
    }
    /// Creates an `ASVariable` from the `KeyVar`.
    pub fn get(&self) -> ASVariable {
        match self {
            Self::Int(c) => ASVariable::Int(*c),
            Self::String(c) => ASVariable::String(c.to_string()),
            Self::Bool(c) => ASVariable::Bool(*c),
        }
    }
}

impl ASVariable {
    /// Returns a `KeyVar` equivalent to the variable. To see what each type means, check
    /// the `ASVariable` documentation.
    pub fn as_key(self) -> anyhow::Result<KeyVar> {
        KeyVar::new(self)
    }
}

impl Display for KeyVar {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.get())
    }
}
