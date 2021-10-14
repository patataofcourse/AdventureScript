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

pub struct TooManyArguments {
    pub script: String,
    pub line: u32,
    pub command: String,
    pub max_args: u32,
    pub given_args: u32,
}

impl ASErr for TooManyArguments {
    fn generic_err(self) -> ASError {
        ASError {
            code: 3,
            script: self.script,
            line: self.line,
            name: String::from("TooManyArguments"),
            message: format!(
                "Command {} takes at most {} non-keyword arguments, but was given {}",
                self.command, self.max_args, self.given_args
            ),
        }
    }
}
