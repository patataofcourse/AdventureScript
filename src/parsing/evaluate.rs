use super::{parse_text, simplify_brackets};
use crate::core::{error::ASSyntaxError, ASVariable, GameInfo, KeyVar, TypeMethods};
use regex::{Match, Regex};
use std::collections::HashMap;

pub fn expr(
    info: &mut GameInfo,
    text: String,
    strings: &Vec<String>,
    brackets: &Vec<String>,
) -> anyhow::Result<ASVariable> {
    let text = text.trim();
    let operator_regex = Regex::new(r"\+|-|\*|/|\^|!=|!|==|>=|<=|<|>")?;
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
            let value = parse_text(info, &brackets[index])?;
            parsed = list(info, value.to_string(), strings)?;
        } else if val.starts_with("{") && val.ends_with("}") {
            let index = ((val.split_at(1).1).split_at(val.len() - 2).0)
                .parse::<usize>()
                .unwrap();
            let value = parse_text(info, &brackets[index])?;
            parsed = if value.contains(":") {
                map(info, value.to_string(), strings)?
            } else {
                let index = ((val.split_at(1).1).split_at(val.len() - 2).0)
                    .parse::<usize>()
                    .unwrap();
                let lname = &brackets[index];
                if !Regex::new(r"^[A-Za-z0-9-_]*$")?.is_match(lname) {
                    Err(ASSyntaxError::InvalidVariableName(val.to_string()))?;
                }
                ASVariable::Label(Some(lname.to_string()))
            }
        } else if val.starts_with("(") && val.ends_with(")") {
            let index = ((val.split_at(1).1).split_at(val.len() - 2).0)
                .parse::<usize>()
                .unwrap();
            let value = parse_text(info, &brackets[index])?;
            let (value, brcks) = simplify_brackets(value)?;
            parsed = expr(info, value, &strings, &brcks)?;
        } else if val.starts_with("\"") && val.ends_with("\"") {
            let index = ((val.split_at(1).1).split_at(val.len() - 2).0)
                .parse::<usize>()
                .unwrap();
            parsed = ASVariable::String(parse_text(info, &strings[index])?);
        } else if val == "None" {
            parsed = ASVariable::None;
        } else if val == "" {
            parsed = ASVariable::Empty;
        }
        //Flags
        else if val.starts_with("?") {
            if !Regex::new(r"^?[A-Za-z0-9-_]*$")?.is_match(&val) {
                Err(ASSyntaxError::InvalidVariableName(val[1..].to_string()))? //TODO: get proper token content
            }
            parsed = ASVariable::VarRef {
                name: val[1..].to_string(),
                flag: true,
            }
        }
        // Variables
        else {
            if !Regex::new(r"^[A-Za-z0-9-_]*$")?.is_match(&val) {
                Err(ASSyntaxError::InvalidVariableName(val.to_string()))? //TODO: get proper token content
            }
            parsed = ASVariable::VarRef {
                name: val.to_string(),
                flag: false,
            }
        }

        // Can't run methods on an Empty type
        if methods.len() > 0 && parsed == ASVariable::Empty {
            Err(ASSyntaxError::InvalidVariableName(val.to_string()))?
        }
        values.push(manage_methods(info, parsed, methods, strings, brackets)?)
    }
    //unary operations
    for operation in vec!["-", "!"] {
        let mut c = 0;
        while c < operators.len() {
            // It's a unary operator if the first value is None
            // Yes this is a dumb way to add it shush
            if operators[c].as_str() == operation && values[c] == ASVariable::Empty {
                operators.remove(c);
                values[c] = match operation {
                    "-" => (-values[c + 1].clone()),
                    "!" => (!values[c + 1].clone()),
                    _ => panic!("unrecognized unary operator"),
                }?;
                values.remove(c + 1);
            } else {
                c += 1;
            }
        }
    }
    //binary operations
    for operation in vec!["^", "*", "/", "+", "-", "==", ">", "<", "!=", ">=", "<="] {
        let mut c = 0;
        while c < operators.len() {
            if operators[c].as_str() == operation {
                operators.remove(c);
                values[c] = match operation {
                    "+" => values[c].clone() + values[c + 1].clone(),
                    "-" => values[c].clone() - values[c + 1].clone(),
                    "*" => values[c].clone() * values[c + 1].clone(),
                    "/" => values[c].clone() / values[c + 1].clone(),
                    "^" => values[c].clone().pow(values[c + 1].clone()),
                    "==" => Ok(ASVariable::Bool(values[c].clone() == values[c + 1].clone())),
                    ">" => Ok(ASVariable::Bool(values[c].clone() > values[c + 1].clone())),
                    "<" => Ok(ASVariable::Bool(values[c].clone() < values[c + 1].clone())),
                    "!=" => Ok(ASVariable::Bool(values[c].clone() != values[c + 1].clone())),
                    ">=" => Ok(ASVariable::Bool(values[c].clone() >= values[c + 1].clone())),
                    "<=" => Ok(ASVariable::Bool(values[c].clone() <= values[c + 1].clone())),
                    _ => panic!("unrecognized operator"),
                }?;
                values.remove(c + 1);
            } else {
                c += 1;
            }
        }
    }
    // Empty type bad
    if values[0] == ASVariable::Empty {
        Err(ASSyntaxError::InvalidVariableName(text.to_string()))?
    }
    Ok(values[0].clone())
}

fn manage_methods(
    info: &mut GameInfo,
    value: ASVariable,
    methods: Vec<String>,
    strings: &Vec<String>,
    brackets: &Vec<String>,
) -> anyhow::Result<ASVariable> {
    let method_regex = Regex::new("^([A-Za-z0-9-_]+)\\s*\\(([0-9]+)\\)$").unwrap();
    let mut value = value;
    for method in &methods {
        let method_captures = method_regex.captures(method);
        if let None = method_captures {
            Err(ASSyntaxError::InvalidMethod(method.to_string()))? //TODO: get proper token content
        } else if let Some(c) = method_regex.captures(method) {
            let bracket = format!("[{}]", c.get(2).unwrap().as_str().parse::<usize>().unwrap());
            let args: Vec<ASVariable>;
            if let ASVariable::List(l) = expr(info, bracket, strings, brackets)? {
                args = l;
            } else {
                panic!()
            }
            value = TypeMethods::get_for_type(info, &value.get_type()).run_method(
                c.get(1).unwrap().as_str(),
                info,
                &value,
                args,
            )?
        };
    }
    Ok(value)
}

fn list(info: &mut GameInfo, text: String, strings: &Vec<String>) -> anyhow::Result<ASVariable> {
    let mut list = vec![];
    let (text, brackets) = simplify_brackets(text)?;

    for elmt in text.split(",") {
        list.push(expr(info, elmt.to_string(), strings, &brackets)?);
    }

    Ok(ASVariable::List(list))
}

fn map(info: &mut GameInfo, text: String, strings: &Vec<String>) -> anyhow::Result<ASVariable> {
    let mut map = HashMap::<KeyVar, ASVariable>::new();
    let (text, brackets) = simplify_brackets(text)?;

    let is_map = Regex::new(":")?;
    for elmt in text.split(",") {
        match is_map.find(&elmt) {
            Some(c) => {
                let (key, value) = elmt.split_at(c.start());
                let key = expr(info, key.to_string(), strings, &brackets)?;
                let value = expr(info, value[1..].to_string(), strings, &brackets)?;

                map.insert(key.as_key()?, value);
            }
            None => Err(ASSyntaxError::MapError)?,
        };
    }

    Ok(ASVariable::Map(map))
}
