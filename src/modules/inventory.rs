use adventure_script_attr::command;

use super::{Module, ObjSpec};
use crate::{
    core::{ASType, KeyVar, TypeMethods},
    unwrap_var,
};

use std::collections::HashMap;

#[command(crate_path="crate", name="test")]
pub fn test_cmd(info: &mut GameInfo) -> anyhow::Result<()> {
    info.show("Test command working!")
}

pub fn get_module(name: Option<&str>) -> Module {
    let name = if let Some(c) = name { c } else { "inv" }.to_string();
    Module::from(
        name.clone(),
        vec![test_cmd().unwrap()],
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
