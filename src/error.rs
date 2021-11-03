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
    } else {
        eprint!("uncaught internal error\n\t");
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
    #[error(
        "Tried to use {feature}, which is unimplemented or a work-in-progress\nDetails: {details}"
    )]
    NotImplementedError {
        feature: String, // Name of the WIP or not implemented feature the user was trying to use
        details: String, // Any extra details, eg: "This will be added for version X"
    },
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

#[derive(Debug)]
pub struct ASSyntaxError {
    pub details: SyntaxErrors,
}

impl Display for ASSyntaxError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "syntax error:\n\t{}", self.details)
    }
}

impl Error for ASSyntaxError {}

#[derive(Debug, Error)]
pub enum SyntaxErrors {
    #[error("Syntax error: {details}")]
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
}
