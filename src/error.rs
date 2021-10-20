use super::variables::ASType;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct ASError {
    pub command: String,
    pub details: CommandErrors,
}

impl Display for ASError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ASError {}

// Error definitions
#[derive(Debug)]
pub enum CommandErrors {
    OtherCommandError {
        details: String,
    },
    NotImplementedError {
        feature: String, // Name of the WIP or not implemented feature the user was trying to use
        details: String, // Any extra details, eg: "This will be added for version X"
    },
    TooManyPosArgs {
        max_args: u32,
        given_args: u32,
    },
    UndefinedArgument {
        argument_name: String,
        argument_type: ASType,
    },
    MissingRequiredArgument {
        argument_name: String,
        argument_type: ASType,
    },
    ArgumentTypeError {
        argument_name: String,
        required_type: ASType,
        given_type: ASType,
    },
}

impl Display for CommandErrors {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use CommandErrors::*;
        match self {
            OtherCommandError { details } => write!(f, "{}", details),
            NotImplementedError { feature, details } => {
                if details != "" {
                    write!(
                        f,
                        "Tried to use {}, which is unimplemented or a work-in-progress\n({})",
                        feature, details
                    )
                } else {
                    write!(
                        f,
                        "Tried to use {}, which is unimplemented or a work-in-progress",
                        feature
                    )
                }
            }
            TooManyPosArgs {
                max_args,
                given_args,
            } => write!(
                f,
                "Command can only take {} positional arguments, but was given {}",
                max_args, given_args
            ),
            UndefinedArgument {
                argument_name,
                argument_type,
            } => write!(
                f,
                "Command was given argument {} (type {}), which it does not recognize",
                argument_name, argument_type
            ),
            MissingRequiredArgument {
                argument_name,
                argument_type,
            } => write!(
                f,
                "Command is missing argument {} (type {}), which is required",
                argument_name, argument_type
            ),
            ArgumentTypeError {
                argument_name,
                given_type,
                required_type,
            } => write!(
                f,
                "Argument {} is of type {}, but type {} was given",
                argument_name, required_type, given_type
            ),
        }
    }
}
