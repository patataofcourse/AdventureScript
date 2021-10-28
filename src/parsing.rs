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

        //TODO: replacing quotes and brackets

        //TODO: comment this
        for arg in text.split(";") {
            let mut must_be_kwarg = false; //args can only be before kwargs
            let mut arg_num = 0; //position for positional args

            let (arg, strings) = simplify(arg.trim().to_string(), SimplifyMode::Strings)?;
            match is_kwarg.find(&arg)? {
                Some(c) => {
                    must_be_kwarg = true;

                    // Split kwarg into argument name (key) and argument body (value)
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

#[test]
fn simplify_test() {
    let text = "'hel\"lo' \"hel'lo\" 'hel\\'lo' \"hel\\\"lo\"".to_string();
    let (s, q) = simplify(text, SimplifyMode::Strings).unwrap();
    assert_eq!(s, "\"0\" \"1\" \"2\" \"3\"");
    assert_eq!(q, vec!["hel\"lo", "hel'lo", "hel\\'lo", "hel\\\"lo"]);
}

// TODO: name better
fn simplify(mut text: String, mode: SimplifyMode) -> anyhow::Result<(String, Vec<String>)> {
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

    // yes this doesn't use regex shut up

    // Step 1:   Get the start and end of every string

    // 1.1:   Get all quotes, both single and double
    let mut quotepos = Vec::<usize>::new(); // here we'll store the index of every quote that's not been escaped
    let mut pos = 0;
    let mut prev_char = '\x00';
    for chr in text.chars() {
        // If the current character is a quote and the character before it isn't a backslash
        // because, well, escaping quotes in strings is A Thing tm
        if (chr == '"' || chr == '\'') && prev_char != '\\' {
            quotepos.push(pos);
        }

        pos += 1;
        prev_char = chr;
    }

    // 1.2:   Scan the code looking for actual strings: opened and closed with the same
    //      type of quote
    let mut prev_char = '\x00'; // reusing variable for storing the currently open
                                // quote type
    let mut quotes = Vec::<(usize, usize)>::new(); // start and end index for the quote

    for index in quotepos {
        let chr = *text.chars().collect::<Vec<char>>().get(index).unwrap();
        match prev_char {
            // If no string is open right now, open a new string
            '\x00' => {
                prev_char = chr;
                quotes.push((index, 0));
            }
            c => {
                // If this char's the same quote type as the opening quote, this is
                // a closing quote
                if c == chr {
                    prev_char = '\x00';
                    let prev_index = quotes.pop().unwrap().0;
                    quotes.push((prev_index, index));
                }
                // Otherwise, treat it as any other character
            }
        }
    }

    // If the string was left unclosed, that's a syntax error
    if prev_char != '\x00' {
        Err(ASSyntaxError {
            details: SyntaxErrors::UnclosedString {},
        })?;
    }

    // Step 2:   Replace the strings with something the parser won't fuck up.
    //         The issue with strings on the AS parser is that they might have symbols that,
    //         unless managed properly, could be interpreted as tokens (+, -, *, /, ;, ...).
    //           So they're replaced with a number (signifying its index in the string Vec)
    //         surrounded by double quotes.

    quotes.reverse(); // Doing this so it won't mess with the other index values

    let mut quotetext = Vec::<String>::new();
    let mut c = 0;
    for quote in &quotes {
        quotetext.push(((text.split_at(quote.1).0).split_at(quote.0 + 1).1).to_string());
        text = format!(
            "{}\"{}\"{}",
            text.split_at(quote.0).0,
            quotes.len() - c - 1, // this makes the indexes be in the right order
            text.split_at(quote.1 + 1).1
        );
        c += 1;
    }

    quotetext.reverse();

    Ok((text, quotetext))
}

fn parse_argument() {}

pub fn evaluate() {}
