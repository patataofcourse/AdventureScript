use std::{io::{stdin, Read}};
use super::error::ASError;

fn show(text: &str) -> Result<(), ASError>{
    println!("{}", text);
    Ok(())
}

fn wait() -> Result<(), ASError>{
    stdin().read(&mut [0]).unwrap();
    Ok(())
}

fn query(text: &str, choices: Vec<&str>, allow_save: bool) -> Result<u8, ASError> {
    if text != "" {
        show(&text);
    }

    let mut c = 1;
    for ch in &choices {
        show(&format!("{}. {}", c, ch));
        c += 1;
    }

    let mut result;
    loop {
        //print!(">");
        result = String::new();
        stdin().read_line(&mut result)
            .expect("Failed to read line");
        match result.trim() {
            "s" => {
                if allow_save {
                    show(&String::from("Would save here"));
                }
            }
            "r" => {
                if allow_save {
                    show(&String::from("Would restore here"));
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
    pub show: fn(&str) -> Result<(), ASError>,
    pub wait: fn() -> Result<(), ASError>,
    pub query: fn(&str, Vec<&str>, bool) -> Result<u8, ASError>,
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
