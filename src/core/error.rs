use crate::core::{ASType, GameInfo, KeyVar};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};
use thiserror::Error;

pub(crate) fn manage_error(info: &GameInfo, err: anyhow::Error) {
    let mut error = format!(
        "\nAdventureScript error on script {}, line {} - ",
        info.script_name(),
        info.pointer(),
    );
    if let Some(_c) = err.downcast_ref::<ASFileError>() {
    } else if let Some(_c) = err.downcast_ref::<ASCmdError>() {
    } else if let Some(_c) = err.downcast_ref::<ASSyntaxError>() {
        error += "syntax error:\n    ";
    } else if let Some(_c) = err.downcast_ref::<ASNotImplemented>() {
        error += "feature not implemented:\n    ";
    } else if let Some(_c) = err.downcast_ref::<ASVarError>() {
        error += "variable error:\n    ";
    } else if let Some(_c) = err.downcast_ref::<ASGameError>() {
        error += "error raised by game:\n    ";
    } else if let Some(_c) = err.downcast_ref::<ASOtherError>() {
        error += "development error:\n    "
    } else {
        error += "uncaught internal error:\n    ";
    };
    info.error(format!(
        "{}{}",
        error,
        (|| {
            let err = err.to_string();
            let mut lines = err.lines();
            let mut out = lines.next().unwrap().to_string();
            for line in lines {
                out += &format!("\n    {}", line);
            }
            out
        })()
    ));
}

// Command error

#[derive(Debug)]
pub struct ASCmdError {
    pub command: String,
    pub details: CommandErrors,
}

impl Display for ASCmdError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "on command {}:\n{}", self.command, self.details)
    }
}

impl Error for ASCmdError {}

#[derive(Debug, Error)]
pub enum CommandErrors {
    #[error("{details}")]
    Generic { details: String },
    #[error("Command can only take {max_args} positional arguments, but was given {given_args}")]
    TooManyPosArgs { max_args: u32, given_args: u32 },
    #[error("Command was given argument {argument_name} (type {argument_type}), which it doesn't recognize")]
    UndefinedArgument {
        argument_name: String,
        argument_type: ASType,
    },
    #[error(
        "Command is missing argument {argument_name} (type {argument_type}), which is required"
    )]
    MissingRequiredArgument {
        argument_name: String,
        argument_type: ASType,
    },
    #[error(
        "Command is missing argument {argument_name} / #{argument_num} {}",
        "(type {argument_type}), which is required"
    )]
    MissingRequiredPosArg {
        argument_name: String,
        argument_num: usize,
        argument_type: ASType,
    },
    #[error(
        "Argument {argument_name} is of type {required_type}, but type {given_type} was given"
    )]
    ArgumentTypeError {
        argument_name: String,
        required_type: ASType,
        given_type: ASType,
    },
    #[error(
        "Argument {argument_name} / #{argument_num} is of type {required_type}, but type {given_type} was given"
    )]
    PosArgTypeError {
        argument_name: String,
        argument_num: usize,
        required_type: ASType,
        given_type: ASType,
    },
    #[error("Choice #{choice} is missing an argument (of type {typ})")]
    ChoiceMissingRequired { choice: u8, typ: ASType },
    #[error("Choice #{choice}, argument {} is of type {asked}, but got {given}", number+1)]
    ChoiceWrongType {
        choice: u8,
        number: u8,
        given: ASType,
        asked: ASType,
    },
}

// File error

#[derive(Debug)]
pub struct ASFileError {
    pub filename: String,
    pub mode: String,
    pub details: FileErrors,
}

impl ASFileError {
    pub fn from(filename: &str, mode: &str, details: FileErrors) -> Self {
        Self {
            filename: filename.to_string(),
            mode: match mode {
                "r" => "reading",
                "w" => "writing",
                _ => "opening",
            }
            .to_string(),
            details,
        }
    }
}

impl Display for ASFileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "error when {} file {}:\n{}",
            self.mode, self.filename, self.details
        )
    }
}

impl Error for ASFileError {}

#[derive(Debug, Error)]
pub enum FileErrors {
    #[error("Mode '{0}' is invalid")]
    InvalidMode(String),
    #[error("File not found")]
    NotFound,
    #[error("Permission denied - check that the file isn't read-only")]
    MissingPermissions,
    #[error("Error parsing configuration file:\n    {0}")]
    ConfigLoadError(String),
    #[error("Error parsing save file:\n    {0}")]
    SaveLoadError(String),
    #[error("Save was created on incompatible AdventureScript version v{0}")]
    SaveNotCompatible(String),
    #[error("Save was created on a newer version of AdventureScript (v{0})")]
    SaveTooNew(String),
}

// Syntax/parsing error

#[derive(Debug, Error)]
pub enum ASSyntaxError {
    #[error("{details}")]
    Generic { details: String },
    #[error("Reached end of script! Add an !ending or !loadscript command")]
    EndOfScript {},
    #[error("Command is empty")]
    NoCommand {},
    #[error("Command !{command} does not exist")]
    NonExistentCommand { command: String },
    #[error("A positional argument was placed after one or more keyword arguments")]
    ArgAfterKwarg {},
    #[error("Unclosed string")]
    UnclosedString {},
    #[error("Unclosed bracket: {bracket}")]
    UnclosedBracket { bracket: char },
    #[error("Can't {op} values of type {type1} to values of type {type2}")]
    OperationNotDefined {
        op: String,
        type1: ASType,
        type2: ASType,
    },
    #[error("Can't {op} value of type {type1}")]
    UnaryOperationNotDefined { op: String, type1: ASType },
    #[error("Escape code {code} does not exist!")]
    InvalidEscapeCode { code: String },
    #[error(
        "Escape code {code} wasn't supplied an argument, which it requires.\nTry using '{code}[...]'"
    )]
    EmptyControlCode { code: String },
    #[error("Failed to parse a map. Likely because of a missing :")]
    MapError,
    #[error("Values of type {key_type} can't be keys in Maps")]
    InvalidMapKey { key_type: ASType },
    #[error(
        "Invalid token {0}\nIf you were trying to name a variable, keep in mind variable names must be made of either alphanumeric characters, dashes, or underscores"
    )]
    InvalidVariableName(String),
    #[error("Label {0} doesn't exist")]
    NonExistentLabel(String),
    #[error("Label {0} is defined multiple times in the following lines: {}",
        ( |vec| {
            let mut out = 1.0.to_string();
            for elmt in [1,2] {
                out += ", ";
                out += &elmt.to_string();
            }
            out
        } ) (.1)
    )]
    RepeatedLabel(String, Vec<i32>),
    #[error(
        "Invalid token {0}\nIf you were trying to name a label, keep in mind label names must be made of either alphanumeric characters, dashes, or underscores"
    )]
    InvalidLabelName(String),
}

// Error for WIP/unimplemented stuff

#[derive(Debug)]
pub struct ASNotImplemented(pub String);

impl Display for ASNotImplemented {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ASNotImplemented {}

//Variable error

#[derive(Debug, Error)]
pub enum ASVarError {
    #[error("Tried to set flag {0} to a non-boolean value")]
    FlagNotBool(String),
    #[error("Tried to access variable {0}, which doesn't exist.\nTip: use !set {0}; [some value]")]
    VarNotFound(String),
    #[error("List indexes should be 0 or positive")]
    NegativeListIndex,
    #[error("Tried to get item {index} of a list that only has {num_items} items")]
    WrongListIndex { num_items: usize, index: i32 },
    #[error("Tried to get value of a map associated to key {key}, which doesn't exist")]
    WrongMapKey { key: KeyVar },
}

//Error raised from the game

#[derive(Debug)]
pub struct ASGameError(pub String);

impl Display for ASGameError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ASGameError {}

//Other unspecified error

#[derive(Debug, Error)]
pub enum ASOtherError {
    #[error("Author of a module left in a bug - please report to them!\n{0}")]
    DevErr(String),
    #[error("You tried running AdventureScript on an unsupported platform")]
    UnsupportedPlatform,
}
