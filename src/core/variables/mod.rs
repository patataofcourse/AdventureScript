use crate::core::error::ASSyntaxError;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    hash::Hash,
};

use self::is_as_var::IsASVar;

mod operations;

#[doc(hidden)]
pub mod is_as_var;

/// Enum listing all possible types for AdventureScript variables. To see what each type means, check
/// the `ASVariable` documentation.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
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

    // for use with command signatures
    #[doc(hidden)]
    ListExplicit(Box<ASType>),
    #[doc(hidden)]
    MapExplicit(Box<ASType>, Box<ASType>),
}

impl Display for ASType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl ASType {
    /// Returns the default value for a variable of the specified type:
    ///
    /// - Any / None /  VarRef / Label => None
    ///     * For VarRef and Label, trying to use this value before it is initialized
    ///     will result in an error
    /// - Bool => false
    /// - Int => 0
    /// - String => ""
    /// - List / Map => empty list or map
    pub fn default_for_type(&self) -> ASVariable {
        match self {
            Self::Any | Self::None | Self::VarRef | Self::Label => ASVariable::None,
            Self::Bool => ASVariable::Bool(false),
            Self::Int => ASVariable::Int(0),
            Self::String => ASVariable::String("".to_string()),
            Self::List => ASVariable::List(vec![]),
            Self::Map => ASVariable::Map(HashMap::new()),
            Self::Object(c) => todo!(), //add object initializer??
            Self::ListExplicit(_) | Self::MapExplicit(..) => unimplemented!(),
        }
    }
}

/// Enum used to handle AdventureScript variables.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ASVariable {
    #[doc(hidden)]
    // For internal use. DO NOT USE in modules
    Empty,

    /// Boolean value (true/false)
    Bool(bool),
    /// Integer value (64-bit signed)
    Int(i64),
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
        /// The name of the object. Used to refer to an object's specification (fields, methods, stringifying, etc.)
        spec: String,
        /// The values contained by the object
        fields: HashMap<String, ASVariable>,
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
            Self::Object { spec, .. } => ASType::Object(spec.clone()),
            Self::Empty => panic!("Cannot use get_type with Empty type"),
        }
    }
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
                Self::String(c) => c.to_string(),
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
                Self::Object { spec, .. } => {
                    format!("<Object type {}>", spec)
                }
                Self::Empty => panic!("Cannot use to_string with Empty type"),
            }
        )
    }
}

/// Struct for `ASVariable` values that can be used as keys in a `Map`.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum KeyVar {
    Bool(bool),
    Int(i64),
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

    pub fn get_type(&self) -> ASType {
        match self {
            Self::Int(_) => ASType::Int,
            Self::String(_) => ASType::String,
            Self::Bool(_) => ASType::Bool,
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
