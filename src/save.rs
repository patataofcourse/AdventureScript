use crate::{
    error::{ASFileError, FileErrors},
    info::GameInfo,
    io::FileType,
    variables::ASVariable,
};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, io::Read, path::PathBuf, str::FromStr};

#[derive(Deserialize, Serialize, Debug)]
pub struct Save {
    //TODO: make versions SemVers
    pub as_ver: String,
    pub game_ver: String,
    pub script: PathBuf,
    //TODO: save position as a label??
    pub pointer: i32,
    pub flags: HashMap<String, bool>,
    pub variables: HashMap<String, ASVariable>,
    //TODO: implement screentext
    pub screentext: String,
}

pub fn test_save() {
    let s = Save {
        as_ver: "a".to_string(),
        game_ver: "b".to_string(),
        script: PathBuf::from_str("c").unwrap(),
        pointer: 3,
        flags: HashMap::new(),
        variables: HashMap::from_iter([(
            "hi".to_string(),
            ASVariable::VarRef {
                name: "s".to_string(),
                flag: false,
            },
        )]),
        screentext: "d".to_string(),
    };
    let a = serde_json::to_string_pretty(&s).unwrap();
    println!("{}", a);
    println!("{:#?}", serde_json::from_str::<Save>(&a));
}
