use super::{Module, ObjSpec};
use crate::{
    command,
    core::{ASType, TypeMethods},
};

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
        vec![ObjSpec {
            name: "Inventory".to_string(),
            fields: HashMap::from([("".to_string(), ASType::Map)]),
            methods: TypeMethods::new(),
        }],
    )
}
