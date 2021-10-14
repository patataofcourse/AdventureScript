use anyhow;
use std::io::{stdin, Read};

fn show(text: &str) -> anyhow::Result<()> {
    println!("{}", text);
    Ok(())
}

fn wait() -> anyhow::Result<()> {
    stdin().read(&mut [0]).unwrap();
    Ok(())
}

fn query(text: &str, choices: Vec<&str>, allow_save: bool) -> anyhow::Result<u8> {
    if !text.is_empty() {
        show(&text)?;
    }

    let mut c = 1;
    for ch in &choices {
        show(&format!("{}. {}", c, ch))?;
        c += 1;
    }

    let mut result;
    loop {
        //print!(">");
        result = String::new();
        stdin().read_line(&mut result).expect("Failed to read line"); //TODO: move this to a separate function
        match result.trim() {
            "s" => {
                if allow_save {
                    show("Would save here")?;
                }
            }
            "r" => {
                if allow_save {
                    show("Would restore here")?;
                }
            }
            "q" => return Ok(0),
            _ => (),
        }

        let num_result: u8 = match result.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if (num_result as usize) <= choices.len() {
            return Ok(num_result);
        }
    }
}

//TODO: add load_file function

pub struct AdventureIO {
    pub show: fn(&str) -> anyhow::Result<()>,
    pub wait: fn() -> anyhow::Result<()>,
    pub query: fn(&str, Vec<&str>, bool) -> anyhow::Result<u8>,
}

impl Default for AdventureIO {
    fn default() -> Self {
        Self {
            show: show,
            wait: wait,
            query: query,
        }
    }
}
