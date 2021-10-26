use super::{
    commands::Command,
    error::{ASSyntaxError, SyntaxErrors},
    info::GameInfo,
    variables::ASVariable,
};
use std::collections::HashMap;

pub fn parse_line(info: &mut GameInfo, commands: &HashMap<String, Command>) -> anyhow::Result<()> {
    let ln = info.get_line()?;
    if ln.starts_with("#") {
        return Ok(());
    } else if ln.starts_with("!") {
        //Since ln[0] is always one byte long, we can use slices
        parse_command(info, commands, ln[1..].trim_start())?;
    } else {
        match ln.as_ref() {
            "\\n" => info.io().show("")?,
            "\\w" => info.io().wait()?,
            "" => return Ok(()),
            _ => {
                let ln = parse_text(info, ln)?;
                info.io().show(&ln)?
            }
        };
    }
    Ok(())
}

pub fn parse_text(info: &mut GameInfo, text: String) -> anyhow::Result<String> {
    //TODO: add proper control code stuff
    Ok(text.replace("\\n", "\n"))
}

pub fn parse_command(
    info: &mut GameInfo,
    commands: &HashMap<String, Command>,
    text: &str,
) -> anyhow::Result<()> {
    // Get the command from the name
    let split = text.split(" ");
    let mut split_vec = Vec::<String>::new();

    for word in split {
        split_vec.push(word.to_string());
    }

    let name = match split_vec.get(0) {
        Some(c) => c,
        None => Err(ASSyntaxError {
            details: SyntaxErrors::NoCommand {},
        })?,
    };

    let command = match commands.get(name) {
        Some(c) => c,
        None => Err(ASSyntaxError {
            details: SyntaxErrors::NonExistentCommand {
                command: name.to_string(),
            },
        })?,
    };

    // Get the arguments
    let args = Vec::<&ASVariable>::new();
    let kwargs = HashMap::<String, &ASVariable>::new();
    if split_vec.len() > 1 {
        let text = split_vec[1..].join(" ");
        //TODO: actually parse the argument text
    }

    command.run(info, args, kwargs)
}
