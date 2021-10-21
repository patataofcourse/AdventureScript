use super::{commands::Command, info::GameInfo, variables::ASVariable};
use std::{collections::HashMap, iter::FromIterator};

pub fn basic_script(info: &mut GameInfo, commands: &Vec<Command>) -> anyhow::Result<()> {
    match info.pointer() {
        1 => (info.get_io().show)("hi"),
        2 => (info.get_io().show)("choice goes right after"),
        3 => commands.get(1).unwrap().run(
            //choice
            info,
            Vec::<&ASVariable>::new(),
            HashMap::<String, &ASVariable>::from_iter([
                (String::from("go1"), &ASVariable::Int(4)),
                (String::from("go2"), &ASVariable::Int(6)),
            ]),
        ),
        4 => (info.get_io().show)("ch1"),
        5 => commands.get(2).unwrap().run(
            //goto
            info,
            Vec::<&ASVariable>::from([&ASVariable::Int(7)]),
            HashMap::new(),
        ),
        6 => (info.get_io().show)("ch2"),
        7 => (info.get_io().show)("bye"),
        8 => commands.get(3).unwrap().run(
            //ending
            info,
            Vec::<&ASVariable>::from([&ASVariable::String("buh bye".to_string())]),
            HashMap::<String, &ASVariable>::new(),
        ),
        _ => (info.get_io().show)("invalid line"),
    }
}

pub fn parse_line(info: &mut GameInfo, commands: &Vec<Command>) -> anyhow::Result<()> {
    Ok(())
}
