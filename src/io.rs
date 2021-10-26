use super::{
    error::{ASFileError, FileErrors},
    info::GameInfo,
};
use anyhow;
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

fn show(text: &str) -> anyhow::Result<()> {
    println!("{}", text);
    Ok(())
}

fn wait() -> anyhow::Result<()> {
    stdin().read(&mut [0])?;
    Ok(())
}

fn input() -> anyhow::Result<String> {
    print!("> ");
    stdout().flush()?;
    let mut result = String::new();
    stdin().read_line(&mut result)?;
    Ok(result)
}

fn load_file(info: &GameInfo, filename: &str, mode: &str) -> anyhow::Result<File> {
    Ok(match mode {
        "r" => File::open(format!("{}/{}", info.root_dir(), filename))?,
        "w" => File::create(format!("{}/{}", info.root_dir(), filename))?,
        _ => Err(ASFileError {
            filename: filename.to_string(),
            mode: mode.to_string(),
            details: FileErrors::InvalidMode {},
        })?,
    })
}

pub struct AdventureIO {
    show: fn(&str) -> anyhow::Result<()>,
    wait: fn() -> anyhow::Result<()>,
    input: fn() -> anyhow::Result<String>,
    load_file: fn(&GameInfo, &str, &str) -> anyhow::Result<File>,
}

impl AdventureIO {
    pub fn show(&self, text: &str) -> anyhow::Result<()> {
        (self.show)(text)
    }
    pub fn wait(&self) -> anyhow::Result<()> {
        (self.wait)()
    }
    pub fn input(&self) -> anyhow::Result<String> {
        (self.input)()
    }
    pub fn load_file(&self, info: &GameInfo, filename: &str, mode: &str) -> anyhow::Result<File> {
        (self.load_file)(info, filename, mode)
    }
}

impl Default for AdventureIO {
    fn default() -> Self {
        Self {
            show: show,
            wait: wait,
            input: input,
            load_file: load_file,
        }
    }
}
