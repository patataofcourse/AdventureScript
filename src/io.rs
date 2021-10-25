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
    pub show: fn(&str) -> anyhow::Result<()>,
    pub wait: fn() -> anyhow::Result<()>,
    pub input: fn() -> anyhow::Result<String>,
    pub load_file: fn(&GameInfo, &str, &str) -> anyhow::Result<File>,
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
