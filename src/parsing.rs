use super::{
    commands::Command,
    error::{ASSyntaxError, SyntaxErrors},
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
        let is_kwarg = Regex::new("(?<=[A-Za-z0-9-_ ])=(?=[A-za-z0-9-_ {\\\"\\[(])")?;

        let (text, strings) = simplify(text, SimplifyMode::Strings)?;
        let (text, brackets) = simplify(text, SimplifyMode::Brackets)?;

        //TODO: comment this
        let mut must_be_kwarg = false; //args can only be before kwargs
        for arg in text.split(";") {
            let arg = arg.trim();

            match is_kwarg.find(arg)? {
                Some(c) => {
                    must_be_kwarg = true;

                    // Split kwarg into argument name (key) and argument body (value)
                    let (name, body) = arg.split_at(c.start());
                    let body = evaluate(info, body[1..].to_string(), &strings, &brackets)?;

                    kwargs.insert(name.to_string(), body);
                }
                None => {
                    if !must_be_kwarg {
                        let arg = evaluate(info, arg.to_string(), &strings, &brackets)?;
                        args.push(arg);
                    } else {
                        Err(ASSyntaxError {
                            details: SyntaxErrors::ArgAfterKwarg {},
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

//TODO: merge this into simplify!!! there's a lot of repeated code here
fn merge_this_into_simplify(mut text: String) -> anyhow::Result<(String, Vec<String>)> {
    // yes this doesn't use regex shut up

    // Step 1:   Get the start and end of every bracketed expression
    let mut brackets = Vec::<(usize, usize)>::new(); // start and end index for the bracket + bracket type
    let mut pos = 0;
    let mut prev_char = '\x00'; // currently open bracket type
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
                let needed_char = match prev_char {
                    '[' => ']',
                    '(' => ')',
                    '{' => '}',
                    //Maybe this should panic?
                    c => Err(ASSyntaxError {
                        details: SyntaxErrors::Generic {
                            details: format!("Bracket type opened unknown: {}", c),
                        },
                    })?,
                };
                if chr == needed_char {
                    prev_char = '\x00';
                    let current = brackets.pop().unwrap();
                    brackets.push((current.0, pos))
                }
            }
        }

        pos += 1;
    }

    // If the bracket was left unclosed, that's a syntax error
    if prev_char != '\x00' {
        Err(ASSyntaxError {
            details: SyntaxErrors::UnclosedBracket { bracket: prev_char },
        })?;
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
    let operator_regex = Regex::new(r"{0}+|-|*|/|^{1}")?;
    let operators = operator_regex.find_iter(&text);
    let raw_vals = operator_regex.captures_iter(&text);
    /*
    operators = re.findall("\+|\-|\*|\/|\^", text)
    raw_values = re.split("\+|\-|\*|\/|\^", text)

    operators = ["**" if i=="^" else i for i in operators]
    operators = ["//" if i=="/" else i for i in operators]

    values = []
    for value in raw_values:
        value, *ops = value.strip().split(".")
        #literals
        if value.isdecimal(): #int literal
            value = int(value)
        elif value.lower() == "true": #bool literal: true
            value = True
        elif value.lower() == "false": #bool literal: false
            value = False
        elif value.startswith('"') and value.endswith('"'): #string literal
            value = outquotes[int(value.strip('"'))]
        elif value.startswith("{") and value.endswith("}"): #label literal
            value = outlabels[int(value.strip("{}"))]
        #saved variables
        elif value.startswith("$"): #list
            value = info.list(value[1:])
        elif value.startswith("%"): #flag
            val = info.flags.get(value[1:], None)
            if val == None:
                val = False
                info.flags[value[1:]] = False
            value = val
        elif value.startswith("&"): #inventory
            if value.startswith("&&"): #default inventory
                try:
                    value = info.inventory
                except AttributeError:
                    raise exceptions.NoDefaultInventoryError(info.scriptname, info.pointer)
            else:
                value = info.inv(value[1:])
        else: #values
            value = info.var(value)
        values.append(repr(await operations.manage_operations(value, ops)))

    for op_groups in ["**"], ["*", "//"], ["+", "-"]:
        c = 0
        while c < len(operators):
            if operators[c] in op_groups:
                op = operators[c]
                operators.pop(c)
                //unary operator should be included here
                values[c] = repr(eval(values[c]+op+values[c+1]))
                values.pop(c+1)
            else:
                c += 1
    if flip_result:
        return -eval(values[0])
    else:
        return eval(values[0])
     */
    Ok(ASVariable::Bool(true))
}
