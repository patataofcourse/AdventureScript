use super::Module;
use crate::command;

use std::collections::HashMap;

pub fn get_module() -> Module {
    Module::from(
        "inv".to_string(),
        vec![command! {
            test () => |info, _kwargs| {
                info.show("Test command working!")
            }
        }],
        HashMap::new(),
    )
}
