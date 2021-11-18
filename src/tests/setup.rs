use crate::*;
use std::collections::HashMap;

fn setup() -> (info::GameInfo, HashMap<String, commands::Command>) {
    (
        info::GameInfo::create(String::from("hello"), io::AdventureIO::default()),
        commands::main_commands(),
    )
}
