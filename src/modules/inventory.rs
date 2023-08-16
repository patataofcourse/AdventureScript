use super::{Module, ObjSpec};
use crate::{
    command_old,
    core::{ASType, KeyVar, TypeMethods},
    unwrap_var,
};

use std::collections::HashMap;

pub fn get_module(name: Option<&str>) -> Module {
    let name = if let Some(c) = name { c } else { "inv" }.to_string();
    Module::from(
        name.clone(),
        vec![command_old! {
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

                if out.is_empty() {
                    "Empty!".to_string()
                } else {
                    out
                }
            },
        }],
        HashMap::from([/*(
            "global".to_string(),
            ASType::Object(format!("{}.Inventory", name)),
        )*/]),
    )
}
