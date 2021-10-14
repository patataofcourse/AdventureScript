pub struct ASError {
    code: u8,
    script: String,
    line: u32,
    name: String,
    message: String
}

//TODO: display code

trait ASErr {
    fn generic_err(self) -> ASError;
}

// Here start the error definitions

pub struct GenericCommandError {
    script: String, line: u32, command: String, details: String,
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
    script: String, line: u32, details: String
}