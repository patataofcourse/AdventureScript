use crate::{
    error::{ASFileError, FileErrors},
    info::GameInfo,
    io::FileType,
    variables::ASVariable,
};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

#[derive(Deserialize, Serialize, Debug)]
pub struct Save {
    //TODO: make versions SemVers
    pub as_ver: String,
    pub game_ver: String,
    //TODO: use PathBufs for filenames
    pub script: String,
    //TODO: save position as a label??
    pub pointer: i32,
    pub flags: HashMap<String, ASVariable>,
    pub variables: HashMap<String, ASVariable>,
    //TODO: implement screentext
    pub screentext: String,
}

pub fn restore(info: &mut GameInfo) -> anyhow::Result<()> {
    let mut file = String::from("");
    info.io()
        //TODO: multisave
        .load_file(info, "save.ad2", "r", FileType::Save)?
        .read_to_string(&mut file)?;
    let save: Save = match serde_json::from_str(&file) {
        Ok(c) => c,
        Err(e) => Err(ASFileError::from(
            "save/save.ad2",
            "r",
            FileErrors::SaveLoadError(e.to_string()),
        ))?,
    };

    //TODO: check versions

    info.load_script(Some(&save.script))?;
    info.pointer = save.pointer;
    info.flags = save.flags;
    info.variables = save.variables;
    //TODO: screentext

    Ok(())
}

pub fn save(info: &GameInfo) -> anyhow::Result<()> {
    Ok(())
}
