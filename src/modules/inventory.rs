use super::Module;
use crate::{command, unwrap_var};

use std::collections::HashMap;

pub fn get_module() -> Module {
    Module::from(
        vec![command! {
            test () => |info, _kwargs| {
                info.show("Test command working!")
            }
        }],
        HashMap::new(),
    )
}
