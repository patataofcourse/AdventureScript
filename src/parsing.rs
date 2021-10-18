use super::{commands, info::GameInfo, variables::ASVariable};
use std::{collections::HashMap, iter::FromIterator};

pub fn basic_script(info: &mut GameInfo) -> anyhow::Result<()> {
    let choice = commands::choice();
    match info.script_data().1 {
        1 => (info.get_io().show)("hi"),
        2 => (info.get_io().show)("choice goes right after"),
        3 => choice.run(
            info,
            Vec::<&ASVariable>::new(),
            HashMap::<String, &ASVariable>::from_iter([
                (String::from("go1"), &ASVariable::Int(4)),
                (String::from("go2"), &ASVariable::Int(6)),
            ]),
        ),
        4 => (info.get_io().show)("ch1"),
        5 => commands::goto_fn(
            info,
            HashMap::from_iter([("pos".to_string(), &ASVariable::Int(7))]),
        ),
        6 => (info.get_io().show)("ch2"),
        7 => (info.get_io().show)("bye"),
        8 => commands::ending_fn(
            info,
            HashMap::from_iter([(
                "name".to_string(),
                &ASVariable::String("buh bye".to_string()),
            )]),
        ),
        _ => (info.get_io().show)("invalid line"),
    }
}
