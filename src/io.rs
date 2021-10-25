use anyhow;
use std::io::{stdin, stdout, Read, Write};

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

//TODO: add load_file function

pub struct AdventureIO {
    pub show: fn(&str) -> anyhow::Result<()>,
    pub wait: fn() -> anyhow::Result<()>,
    pub input: fn() -> anyhow::Result<String>,
}

impl Default for AdventureIO {
    fn default() -> Self {
        Self {
            show: show,
            wait: wait,
            input: input,
        }
    }
}
