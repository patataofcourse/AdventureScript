use super::{Module, ObjSpec};
use crate::{
    command,
    core::{ASType, KeyVar, TypeMethods},
    unwrap_var,
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
            fields: HashMap::from([("inv".to_string(), ASType::Map)]),
            methods: TypeMethods::new(),
            stringify: |fields| {
                let inv = unwrap_var!(fields -> "inv"; Map).unwrap();
                let mut out = "".to_string();
                for (key, value) in inv {
                    if key.clone() == KeyVar::String("".to_string()) {
                        continue;
                    }
                    out += &format!("- {} x{}\n", key, value);
                }
                out = out.trim().to_string();

                if out == "" {
                    "Empty!".to_string()
                } else {
                    out
                }
            },
        }],
    )
}
