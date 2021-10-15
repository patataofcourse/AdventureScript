use super::variables::ASType;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ASError {
    code: u32,
    script: String,
    line: u32,
    name: String,
    message: String,
}

impl fmt::Display for ASError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AdventureScript error {} ({}) on script {}, line {}\n{}",
            self.code, self.name, self.script, self.line, self.message
        )
    }
}

impl Error for ASError {}

pub trait ASErr {
    fn generic_err(self) -> ASError;
}

// Here start the error definitions

pub struct GenericCommandError {
    pub script: String,
    pub line: u32,
    pub command: String,
    pub details: String,
}

impl ASErr for GenericCommandError {
    fn generic_err(self) -> ASError {
        ASError {
            code: 1,
            script: self.script,
            line: self.line,
            name: String::from("GenericCommandError"),
            message: format!("Error on {} command: {}", self.command, self.details),
        }
    }
}

pub struct NotImplementedError {
    pub script: String,
    pub line: u32,
    pub details: String,
}

impl ASErr for NotImplementedError {
    fn generic_err(self) -> ASError {
        ASError {
            code: 2,
            script: self.script,
            line: self.line,
            name: String::from("NotImplementedError"),
            message: self.details,
        }
    }
}

pub struct TooManyPositionalArguments {
    pub script: String,
    pub line: u32,
    pub command: String,
    pub max_args: u32,
    pub given_args: u32,
}

impl ASErr for TooManyPositionalArguments {
    fn generic_err(self) -> ASError {
        ASError {
            code: 3,
            script: self.script,
            line: self.line,
            name: String::from("TooManyPositionalArguments"),
            message: format!(
                "Command {} takes at most {} positional arguments, but was given {}",
                self.command, self.max_args, self.given_args
            ),
        }
    }
}

pub struct UndefinedArgument {
    pub script: String,
    pub line: u32,
    pub command: String,
    pub argument_name: String,
    pub argument_type: ASType,
}

impl ASErr for UndefinedArgument {
    fn generic_err(self) -> ASError {
        ASError {
            code: 4,
            script: self.script,
            line: self.line,
            name: String::from("UndefinedArgument"),
            message: format!(
                "Command {} was given argument {} (type {:?}), which it doesn't take",
                self.command, self.argument_name, self.argument_type
            ),
        }
    }
}

pub struct MissingRequiredArgument<'a> {
    pub script: String,
    pub line: u32,
    pub command: String,
    pub argument_name: String,
    pub argument_type: &'a ASType,
}

impl ASErr for MissingRequiredArgument<'_> {
    fn generic_err(self) -> ASError {
        ASError {
            code: 4,
            script: self.script,
            line: self.line,
            name: String::from("MissingRequiredArgument"),
            message: format!(
                "Command {} requires argument {} (type {:?}), which it didn't get",
                self.command, self.argument_name, self.argument_type
            ),
        }
    }
}

pub struct ArgumentTypeError<'a> {
    pub script: String,
    pub line: u32,
    pub command: String,
    pub argument_name: String,
    pub given_type: ASType,
    pub argument_type: &'a ASType,
}

impl ASErr for ArgumentTypeError<'_> {
    fn generic_err(self) -> ASError {
        ASError {
            code: 4,
            script: self.script,
            line: self.line,
            name: String::from("ArgumentTypeError"),
            message: format!(
                "Argument {} for command {} is type {:?}, but got {:?}",
                self.argument_name, self.command, self.argument_type, self.given_type
            ),
        }
    }
}
