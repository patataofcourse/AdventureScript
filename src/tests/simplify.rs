use crate::{core::ASVariable, *};
use std::collections::HashMap;

mod setup;

#[test]
fn goto() {
    let (mut info, commands) = setup::setup();
    let goto = commands.get("goto").expect("No command goto");
    goto.run(
        &mut info,
        Vec::<ASVariable>::from([ASVariable::Int(7)]),
        HashMap::<String, ASVariable>::new(),
    )
    .expect("Error on running command");
    assert_eq!(6, info.pointer());
}
