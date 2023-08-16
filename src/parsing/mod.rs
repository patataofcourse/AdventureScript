use crate::core::{error::ASSyntaxError, ASVariable, CmdSet, GameInfo};
use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

mod evaluate;

pub fn parse_line(info: &mut GameInfo, commands: &CmdSet) -> anyhow::Result<()> {
    let mut ln = info.get_line()?.to_string();
    while ln.starts_with("!!") {
        info.pointer -= 1;
        ln = info.get_line()?.to_string();
    }
    if ln.starts_with('#') || (ln.starts_with('{') && ln.trim().ends_with('}')) {
    } else if let Some(ln) = ln.strip_prefix('!') {
        //TODO: disallow multiline strings
        let mut c = 1;
        let mut ln = ln[1..].trim().to_owned();
        while let Some(next_ln) = info.line_at(info.pointer + c) {
            if let Some(next_ln) = next_ln.strip_prefix("!!>") {
                ln += &format!("[{}];", next_ln.trim());
            } else if let Some(next_ln) = next_ln.strip_prefix("!!>") {
                ln += &format!(" {}", next_ln.trim());
            } else {
                break;
            }

            c += 1
        }
        ln = ln.trim_end().trim_end_matches(';').to_string();
        info.pointer += c - 1;
        parse_command(info, commands, ln)?;
    } else {
        match ln.as_ref() {
            "\\n" => info.show("")?,
            "" => return Ok(()),
            _ => {
                let ln = parse_text(info, &ln)?;
                info.show(&ln)?
            }
        };
    }
    Ok(())
}

fn parse_text(info: &mut GameInfo, text: &str) -> anyhow::Result<String> {
    let control_code_regex = Regex::new(r"\\(.)(\[([^\[]*|\[.*\])\])?")?;
    let control_codes = control_code_regex.captures_iter(text);

    let mut text = text.to_string();
    for capture in control_codes {
        let code = capture.get(1).unwrap().as_str();
        text = control_code_regex
            .replace(
                &text,
                match code {
                    "n" => "\n".to_string(),
                    r"\" => r"\".to_string(),
                    "v" => {
                        let (tx, strings) = simplify_strings(
                            match capture.get(3) {
                                Some(c) => c,
                                None => Err(ASSyntaxError::EmptyControlCode {
                                    code: "\\v".to_string(),
                                })?,
                            }
                            .as_str()
                            .to_string(),
                        )?;
                        let (tx, brackets) = simplify_brackets(tx)?;
                        let result = evaluate::expr(info, tx, &strings, &brackets)?;
                        if let ASVariable::VarRef { .. } = result.clone() {
                            info.get_var(&result)?.to_string()
                        } else {
                            result.to_string()
                        }
                    }
                    c => Err(ASSyntaxError::InvalidEscapeCode {
                        code: format!("\\{}", c),
                    })?,
                },
            )
            .to_string();
    }

    Ok(text)
}

// part 1 of the proper parser code - spoiler alert it's bad
fn parse_command(info: &mut GameInfo, commands: &CmdSet, text: String) -> anyhow::Result<()> {
    // Get the command from the name
    let split: Vec<&str> = text.split(' ').collect();

    let name = match split.first() {
        Some(c) => c,
        None => Err(ASSyntaxError::NoCommand {})?,
    };

    let command = match commands.get(name) {
        Some(c) => c,
        None => Err(ASSyntaxError::NonExistentCommand {
            command: name.to_string(),
        })?,
    };

    // Get the arguments
    let mut args = Vec::<ASVariable>::new();
    let mut kwargs = HashMap::<String, ASVariable>::new();
    if split.len() > 1 {
        let text = split[1..].join(" ");

        // Regex for detecting kwargs ('key=value' format)
        //
        // Since conditional operations are a thing now, it has to check it's not one,
        // and it's honestly just easier to check that the char to the left is a space
        // or proper variable name char, and the one to the right is that or an opening
        // bracket (since those are gonna be evaluated too)
        let is_kwarg = Regex::new("^[A-Za-z0-9-_]*\\s*(=)\\s*[A-za-z0-9-_ {\"'\\[(?]")?;
        let (text, strings) = simplify_strings(text)?;
        let (text, brackets) = simplify_brackets(text)?;

        let mut must_be_kwarg = false; //args can only be before kwargs
        for arg in text.split(';') {
            let arg = arg.trim();

            match is_kwarg.captures(arg) {
                Some(c) => {
                    // Is a keyword argument
                    must_be_kwarg = true;

                    // Split kwarg into argument name (key) and argument body (value)
                    let (name, body) = arg.split_at(c.get(1).unwrap().start());
                    let name = name.trim();
                    let body = evaluate::expr(info, body[1..].to_string(), &strings, &brackets)?;

                    kwargs.insert(name.to_string(), body);
                }
                None => {
                    // Is a positional argument
                    if !must_be_kwarg {
                        let arg = evaluate::expr(info, arg.to_string(), &strings, &brackets)?;
                        args.push(arg);
                    } else {
                        // Positional arguments can't be placed after keyword arguments
                        Err(ASSyntaxError::ArgAfterKwarg {})?;
                    }
                }
            }
        }
    }
    command.run(info, args, kwargs)
}

fn simplify_strings(mut text: String) -> anyhow::Result<(String, Vec<String>)> {
    // yes this doesn't use regex shut up

    // Step 1:   Get the start and end of every string

    // 1.1:   Get all quotes, both single and double
    let mut quotepos = Vec::<usize>::new(); // here we'll store the index of every quote that's not been escaped
    let mut prev_char = '\x00';
    for (pos, chr) in text.chars().enumerate() {
        // If the current character is a quote and the character before it isn't a backslash
        // because, well, escaping quotes in strings is A Thing tm
        if (chr == '"' || chr == '\'') && prev_char != '\\' {
            quotepos.push(pos);
        }

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
        Err(ASSyntaxError::UnclosedString {})?;
    }

    // Step 2:   Replace the strings with something the parser won't fuck up.
    //         The issue with strings on the AS parser is that they might have symbols that,
    //         unless managed properly, could be interpreted as tokens (+, -, *, /, ;, ...).
    //           So they're replaced with a number (signifying its index in the string Vec)
    //         surrounded by double quotes.

    quotes.reverse(); // Doing this so it won't mess with the other index values

    let mut quotetext = Vec::<String>::new();
    for (c, quote) in quotes.iter().enumerate() {
        quotetext.push(((text.split_at(quote.1).0).split_at(quote.0 + 1).1).to_string());
        text = format!(
            "{}\"{}\"{}",
            text.split_at(quote.0).0,
            quotes.len() - c - 1, // this makes the indexes be in the right order
            text.split_at(quote.1 + 1).1
        );
    }

    quotetext.reverse();

    Ok((text, quotetext))
}

fn simplify_brackets(mut text: String) -> anyhow::Result<(String, Vec<String>)> {
    // yes this doesn't use regex shut up

    // Step 1:   Get the start and end of every bracketed expression
    let mut brackets = Vec::<(usize, usize)>::new(); // start and end index for the bracket + bracket type
    let mut pos = 0;
    let mut prev_char = '\x00'; // currently open bracket type
    let mut nesting = 0;
    for chr in text.chars() {
        match prev_char {
            //no opened brackets
            '\x00' => {
                if chr == '[' || chr == '{' || chr == '(' {
                    prev_char = chr;
                    brackets.push((pos, 0));
                }
            }
            _ => {
                if chr == prev_char {
                    nesting += 1;
                    pos += 1;
                    continue;
                }
                let needed_char = match prev_char {
                    '[' => ']',
                    '(' => ')',
                    '{' => '}',
                    _ => panic!(),
                };
                if chr == needed_char {
                    if nesting > 0 {
                        nesting -= 1;
                    } else {
                        prev_char = '\x00';
                        let current = brackets.pop().unwrap();
                        brackets.push((current.0, pos))
                    }
                }
            }
        }

        pos += 1;
    }

    // If the bracket was left unclosed, that's a syntax error
    if prev_char != '\x00' {
        Err(ASSyntaxError::UnclosedBracket { bracket: prev_char })?;
    }

    // Step 2:   Replace the brackets with something the parser won't interpret.

    brackets.reverse(); // Doing this so it won't mess with the other index values

    let mut brackettext = Vec::<String>::new();
    for (c, bracket) in brackets.iter().enumerate() {
        brackettext.push(((text.split_at(bracket.1).0).split_at(bracket.0 + 1).1).to_string());
        text = format!(
            "{}{}{}",
            text.split_at(bracket.0 + 1).0,
            brackets.len() - c - 1, // this makes the indexes be in the right order
            text.split_at(bracket.1).1
        );
    }

    brackettext.reverse();

    Ok((text, brackettext))
}
