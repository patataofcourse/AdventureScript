use super::{variables::ASVariable, *};
use std::collections::HashMap;

fn setup() -> (info::GameInfo, HashMap<String, commands::Command>) {
    (
        info::GameInfo::create(String::from("hello"), io::AdventureIO::default()),
        commands::main_commands(),
    )
}

#[test]
fn goto() {
    let (mut info, commands) = setup();
    let goto = commands.get("goto").expect("No command goto");
    goto.run(
        &mut info,
        Vec::<ASVariable>::from([ASVariable::Int(7)]),
        HashMap::<String, ASVariable>::new(),
    )
    .expect("Error on running command");
    assert_eq!(6, info.pointer());
}
