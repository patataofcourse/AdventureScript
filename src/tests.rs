use super::{variables::ASVariable, *};
use std::collections::HashMap;

fn setup() -> (info::GameInfo, Vec<commands::Command>) {
    (
        info::GameInfo::create(io::AdventureIO::default(), String::from("hello")),
        commands::main_commands(),
    )
}

#[test]
fn goto() {
    let (mut info, commands) = setup();
    let goto = commands.get(2).expect("No command 1 (goto)");
    goto.run(
        &mut info,
        Vec::<&ASVariable>::from([&ASVariable::Int(7)]),
        HashMap::<String, &ASVariable>::new(),
    )
    .expect("Error on run command");
    assert_eq!(6, info.pointer());
}
