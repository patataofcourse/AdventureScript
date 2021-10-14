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

trait ASErr {
    fn generic_err(self) -> ASError;
}

// Here start the error definitions

pub struct GenericCommandError {
    script: String,
    line: u32,
    command: String,
    details: String,
}

impl ASErr for GenericCommandError {
    fn generic_err(self) -> ASError {
        ASError {
            code: 1,
            script: self.script,
            line: self.line,
            name: String::from("GenericCommandError"),
            message: self.details,
        }
    }
}

pub struct NotImplementedError {
    script: String,
    line: u32,
    details: String,
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
