use super::{
    commands::Command,
    error::{ASCmdError, ASSyntaxError, CommandErrors, SyntaxErrors},
    info::GameInfo,
    variables::ASVariable,
};
use fancy_regex::Regex;
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

fn parse_text(info: &mut GameInfo, text: String) -> anyhow::Result<String> {
    //TODO: add proper control code stuff
    Ok(text.replace("\\n", "\n"))
}

// part 1 of the proper parser code - spoiler alert it's bad
fn parse_command(
    info: &mut GameInfo,
    commands: &HashMap<String, Command>,
    text: &str,
) -> anyhow::Result<()> {
    // Get the command from the name
    let split: Vec<&str> = text.split(" ").collect();

    let name = match split.get(0) {
        Some(c) => c,
        None => Err(ASSyntaxError {
            details: SyntaxErrors::NoCommand {},
        })?,
    };

    let command = match commands.get(*name) {
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
    if split.len() > 1 {
        let text = split[1..].join(" ");

        // Regex for detecting kwargs ('key=value' format)
        //
        // Since conditional operations are a thing now, it has to check it's not one,
        // and it's honestly just easier to check that the char to the left is a space
        // or proper variable name char, and the one to the right is that or an opening
        // bracket (since those are gonna be evaluated too)
        let is_kwarg = Regex::new(r"(?<=[A-Za-z0-9-_ ])=(?=[A-za-z0-9-_ {\[(])")?;

        for arg in text.split(";") {
            let mut must_be_kwarg = false; //args can only be before kwargs
            let mut arg_num = 0; //position for positional args

            let arg = arg.trim();
            match is_kwarg.find(arg)? {
                Some(c) => {
                    must_be_kwarg = true;

                    // Split kwarg into argument name (key) and argument body (value)
                    let pos = c.start();
                    let (name, body) = arg.split_at(c.start());
                    let body = body[1..].to_string();

                    //TODO: manage this
                }
                None => {
                    if !must_be_kwarg {
                        //TODO: manage this
                        arg_num += 1;
                    } else {
                        Err(ASSyntaxError {
                            details: SyntaxErrors::ArgAfterKwarg {
                                arg: arg.to_string(),
                            },
                        })?;
                    }
                }
            }
        }
    }

    command.run(info, args, kwargs)
}

enum SimplifyMode {
    Strings,
    Brackets,
}

// TODO: name better :D
// This function was ported from python lmao
fn simplify(text: String, mode: SimplifyMode) -> anyhow::Result<(String, Vec<String>)> {
    //TODO: implement Brackets mode
    match mode {
        SimplifyMode::Strings => (),
        SimplifyMode::Brackets => Err(ASCmdError {
            command: "none/parser".to_string(),
            details: CommandErrors::NotImplementedError {
                feature: "bracket simplifying".to_string(),
                details: "didnt feel like it ngl".to_string(),
            },
        })?,
    }

    // Step 1: Get the start and end of every string

    // Get all quotes, both single and double
    let mut quotepos = Vec::<usize>::new(); // here we'll store the index of every quote that's not been escaped
    for quote in ['\'', '\"'] {
        let mut pos = 0;
        let mut prev_char = '\x00';
        for chr in text.chars() {
            // If the current character is a quote and the character before it isn't a backslash
            // because, well, escaping quotes in strings is A Thing tm
            if (chr == '"' || chr == '\'') && prev_char != '\\' {
                quotepos.push(pos);
            }
        }
    }

    Ok(("".to_string(), vec!["".to_string()]))
    /*    allpos = [i for i in range(len(text)) if text.startswith(quote, i)] #gets all instances of each type of quotes
        for index in allpos:
            if text[index-1] != "\\":
                quotepos.append(index) #only pass to quotepos the strings that weren't escaped
    opened_quote = ""
    quotes = []

    for index in sorted(quotepos):
        if opened_quote == "": #no open quotes
            opened_quote = text[index]
            quotes.append(index)
        else if opened_quote == text[index]:       #current quote is the same as the open quote -> it closes, and
            quotes[-1] = (quotes[-1], index)    #otherwise it just gets ignored and treated as any other character
            opened_quote = ""
    if opened_quote != "":
        raise SyntaxError(f"AdventureScript syntax: unclosed {opened_quote}")
    #now, replace them with things that won't be screwed up by the rest of input_format
    quotes.reverse() #this way the index numbers don't get fucked up
    c = 1
    quotetext = []
    for quote in quotes:
        quotetext = [text[quote[0]+1:quote[1]]] + quotetext
        text = text[:quote[0]] + f'"{len(quotes)-c}"' + text[quote[1]+1:] #"0", "1", etc.
        c += 1
    outquotes = [i.replace("\\'", "'").replace('\\"', '"') for i in quotetext] #gets all instances of each type of quotes
    return text, outquotes*/
}

fn parse_argument() {}

pub fn evaluate() {}
