use super::{
    commands::Command,
    error::{ASNotImplemented, ASSyntaxError},
    info::GameInfo,
    variables::ASVariable,
};
use fancy_regex::Regex as FancyRegex;
use regex::{Match, Regex};
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
    let control_code_regex = Regex::new(r"\\(.)(\[([^\[]*|\[.*\])\])?")?;
    let control_codes = control_code_regex.captures_iter(&text);

    let mut text = text.clone();
    for capture in control_codes {
        let code = capture.get(1).unwrap().as_str();
        text = control_code_regex
            .replace(
                &text,
                match code {
                    "n" => "\n".to_string(),
                    r"\" => r"\".to_string(),
                    "v" => {
                        let (tx, strings) = simplify(
                            match capture.get(3) {
                                Some(c) => c,
                                None => Err(ASSyntaxError::EmptyControlCode {
                                    code: code.to_string(),
                                })?,
                            }
                            .as_str()
                            .to_string(),
                            SimplifyMode::Strings,
                        )?;
                        let (tx, brackets) = simplify(tx, SimplifyMode::Brackets)?;
                        evaluate(info, tx, &strings, &brackets)?.to_string()
                    }
                    c => Err(ASSyntaxError::InvalidEscapeCode {
                        code: c.to_string(),
                    })?,
                },
            )
            .to_string();
    }

    Ok(text)

    //TODO: - use regex and a match
    //      - manage \v(...)
    //      - error if unknown code
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
        None => Err(ASSyntaxError::NoCommand {})?,
    };

    let command = match commands.get(*name) {
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
        let is_kwarg = FancyRegex::new("(?<=[A-Za-z0-9-_ ])=(?=[A-za-z0-9-_ {\\\"\\[(])")?;

        let (text, strings) = simplify(text, SimplifyMode::Strings)?;
        let (text, brackets) = simplify(text, SimplifyMode::Brackets)?;

        let mut must_be_kwarg = false; //args can only be before kwargs
        for arg in text.split(";") {
            let arg = arg.trim();

            match is_kwarg.find(arg)? {
                Some(c) => {
                    // Is a keyword argument
                    must_be_kwarg = true;

                    // Split kwarg into argument name (key) and argument body (value)
                    let (name, body) = arg.split_at(c.start());
                    let body = evaluate(info, body[1..].to_string(), &strings, &brackets)?;

                    kwargs.insert(name.to_string(), body);
                }
                None => {
                    // Is a positional argument
                    if !must_be_kwarg {
                        let arg = evaluate(info, arg.to_string(), &strings, &brackets)?;
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
        SimplifyMode::Brackets => return merge_this_into_simplify(text),
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
        Err(ASSyntaxError::UnclosedString {})?;
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

//TODO: merge this into simplify!!! there's a lot of repeated code here
fn merge_this_into_simplify(mut text: String) -> anyhow::Result<(String, Vec<String>)> {
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
                    //Maybe this should panic?
                    c => Err(ASSyntaxError::Generic {
                        details: format!("Bracket type opened unknown: {}", c),
                    })?,
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
    let mut c = 0;
    for bracket in &brackets {
        brackettext.push(((text.split_at(bracket.1).0).split_at(bracket.0 + 1).1).to_string());
        text = format!(
            "{}{}{}",
            text.split_at(bracket.0 + 1).0,
            brackets.len() - c - 1, // this makes the indexes be in the right order
            text.split_at(bracket.1).1
        );
        c += 1;
    }

    brackettext.reverse();

    Ok((text, brackettext))
}

pub fn evaluate(
    info: &mut GameInfo,
    text: String,
    strings: &Vec<String>,
    brackets: &Vec<String>,
) -> anyhow::Result<ASVariable> {
    let text = text.trim();
    let operator_regex = Regex::new(r"\+|-|\*|/|\^")?;
    let mut operators = operator_regex.find_iter(&text).collect::<Vec<Match>>();
    let raw_vals = operator_regex.split(&text);

    let mut values = Vec::<ASVariable>::new();
    for v in raw_vals {
        let mut val: Option<String> = None;
        let mut methods = vec![];
        for expr in v.split(".") {
            match val {
                None => val = Some(expr.trim().to_string()),
                Some(_) => methods.push(expr.trim().to_string()),
            }
        }
        let val = val.unwrap();
        let parsed: ASVariable;
        // Literals
        if val.parse::<i32>().is_ok() {
            parsed = ASVariable::Int(val.parse::<i32>().unwrap());
        } else if val == "true" || val == "True" {
            parsed = ASVariable::Bool(true);
        } else if val == "false" || val == "False" {
            parsed = ASVariable::Bool(false);
        } else if val.starts_with("[") && val.ends_with("]") {
            let index = ((val.split_at(1).1).split_at(val.len() - 2).0)
                .parse::<usize>()
                .unwrap();
            let value = parse_text(info, brackets[index].clone())?;
            parsed = eval_list(info, value.to_string(), strings)?;
        } else if val.starts_with("{") && val.ends_with("}") {
            //TODO: manage labels and maps
            parsed = ASVariable::None;
            Err(ASNotImplemented {
                details: "Label and Map type literals".to_string(),
            })?;
        } else if val.starts_with("(") && val.ends_with(")") {
            let index = ((val.split_at(1).1).split_at(val.len() - 2).0)
                .parse::<usize>()
                .unwrap();
            let value = parse_text(info, brackets[index].clone())?;
            let (value, brcks) = simplify(value, SimplifyMode::Brackets)?;
            parsed = evaluate(info, value, &strings, &brcks)?;
        } else if val.starts_with("\"") && val.ends_with("\"") {
            let index = ((val.split_at(1).1).split_at(val.len() - 2).0)
                .parse::<usize>()
                .unwrap();
            parsed = ASVariable::String(parse_text(info, strings[index].clone())?);
        } else if val == "None" || val == "" {
            parsed = ASVariable::None;
        }
        //TODO: Variables
        else {
            parsed = ASVariable::None;
            Err(ASNotImplemented {
                details: "Variables".to_string(),
            })?;
        }
        values.push(manage_methods(info, parsed, methods, strings, brackets)?)
    }
    //unary operations (currently only Neg)
    for operation in vec!["-"] {
        let mut c = 0;
        while c < operators.len() {
            // It's a unary operator if the first value is None
            // Yes this is a dumb way to add it shush
            if operators[c].as_str() == operation && values[c] == ASVariable::None {
                operators.remove(c);
                values[c] = match operation {
                    "-" => (-values[c + 1].clone()),
                    _ => panic!("unrecognized unary operator"),
                }?;
                values.remove(c + 1);
            } else {
                c += 1;
            }
        }
    }
    //binary operations
    for operation in vec!["^", "*", "/", "+", "-"] {
        let mut c = 0;
        while c < operators.len() {
            if operators[c].as_str() == operation {
                operators.remove(c);
                values[c] = match operation {
                    "+" => (values[c].clone() + values[c + 1].clone()),
                    "-" => (values[c].clone() - values[c + 1].clone()),
                    "*" => (values[c].clone() * values[c + 1].clone()),
                    "/" => (values[c].clone() / values[c + 1].clone()),
                    "^" => values[c].clone().pow(values[c + 1].clone()),
                    _ => panic!("unrecognized operator"),
                }?;
                values.remove(c + 1);
            } else {
                c += 1;
            }
        }
    }
    Ok(values[0].clone())
}

//TODO: methods
fn manage_methods(
    _info: &GameInfo,
    value: ASVariable,
    _methods: Vec<String>,
    _strings: &Vec<String>,
    _brackets: &Vec<String>,
) -> anyhow::Result<ASVariable> {
    Ok(value)
}

fn eval_list(
    info: &mut GameInfo,
    text: String,
    strings: &Vec<String>,
) -> anyhow::Result<ASVariable> {
    let mut list = vec![];
    let (text, brackets) = simplify(text, SimplifyMode::Brackets)?;

    for elmt in text.split(",") {
        list.push(evaluate(info, elmt.to_string(), strings, &brackets)?);
    }

    Ok(ASVariable::List(list))
}
