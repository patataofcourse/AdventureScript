use super::{info::GameInfo, variables::ASType};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};
use thiserror::Error;

pub fn manage_error(info: &GameInfo, err: anyhow::Error) {
    eprint!(
        "\nAdventureScript error on script {}, line {} - ",
        info.script_name(),
        info.pointer(),
    );
    if let Some(_c) = err.downcast_ref::<ASFileError>() {
    } else if let Some(_c) = err.downcast_ref::<ASCmdError>() {
    } else if let Some(_c) = err.downcast_ref::<ASSyntaxError>() {
        eprint!("syntax error:\n\t")
    } else if let Some(_c) = err.downcast_ref::<ASNotImplemented>() {
    } else {
        eprint!("uncaught internal error:\n\t");
    };
    eprintln!("{}", err);
}

// Command error

#[derive(Debug)]
pub struct ASCmdError {
    pub command: String,
    pub details: CommandErrors,
}

impl Display for ASCmdError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "on command {}:\n\t{}", self.command, self.details)
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
        "Argument {argument_name} is of type {required_type}, but type {given_type} was given"
    )]
    ArgumentTypeError {
        argument_name: String,
        required_type: ASType,
        given_type: ASType,
    },
}

// File error

#[derive(Debug)]
pub struct ASFileError {
    pub filename: String,
    pub mode: String,
    pub details: FileErrors,
}

impl Display for ASFileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "error when opening file {} with mode '{}':\n\t{}",
            self.filename, self.mode, self.details
        )
    }
}

impl Error for ASFileError {}

#[derive(Debug, Error)]
pub enum FileErrors {
    #[error("Mode given is invalid")]
    InvalidMode {},
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
        "Escape code {code} wasn't supplied an argument, which it requires.\nTry using '\\{code}[...]'"
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

#[derive(Debug)]
pub struct ASNotImplemented(pub String);

impl Display for ASNotImplemented {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "feature not implemented:\n\t{}", self.0)
    }
}

impl Error for ASNotImplemented {}

#[derive(Debug, Error)]
pub enum ASVarError {
    #[error("Tried to set flag {0} to a non-boolean value")]
    FlagNotBool(String),
    #[error("Tried to access variable {0}, which doesn't exist.\nTip: use !set {0}; [some value]")]
    VarNotFound(String),
}
