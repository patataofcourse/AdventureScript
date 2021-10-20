use super::{info::GameInfo, variables::ASVariable};
use std::{collections::HashMap, iter::FromIterator};

pub fn basic_script(info: &mut GameInfo) -> anyhow::Result<()> {
    match info.script_data().1 {
        1 => (info.get_io().show)("hi"),
        2 => (info.get_io().show)("choice goes right after"),
        /*3 => info.commands().get(1).unwrap().run(
            //choice
            info,
            Vec::<&ASVariable>::new(),
            HashMap::<String, &ASVariable>::from_iter([
                (String::from("go1"), &ASVariable::Int(4)),
                (String::from("go2"), &ASVariable::Int(6)),
            ]),
        ),
        4 => (info.get_io().show)("ch1"),
        5 => info.commands().get(2).unwrap().run(
            //goto
            info,
            Vec::<&ASVariable>::from([&ASVariable::Int(7)]),
            HashMap::new(),
        ),
        6 => (info.get_io().show)("ch2"),
        7 => (info.get_io().show)("bye"),
        8 => info.commands().get(3).unwrap().run(
            //ending
            info,
            Vec::<&ASVariable>::from([&ASVariable::String("buh bye".to_string())]),
            HashMap::<String, &ASVariable>::new(),
        ),*/
        _ => (info.get_io().show)("invalid line"),
    }
}
