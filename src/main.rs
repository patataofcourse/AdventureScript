use adventure_script::{commands, info::GameInfo, io::AdventureIO};
use std::collections::HashMap;

fn main() {
    let info = GameInfo::create(AdventureIO::default(), "some name");
    commands::input(info, HashMap::new());
}
