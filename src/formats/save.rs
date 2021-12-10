use crate::core::{
    error::{ASFileError, FileErrors},
    ASVariable, FileType, GameInfo,
};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read, io::Write};

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
    info
        //TODO: multisave
        .load_file("save.ad2", "r", FileType::Save)?
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
    info.screentext = save.screentext;
    info.show_screentext()?;

    Ok(())
}

pub fn save(info: &GameInfo) -> anyhow::Result<()> {
    let save = serde_json::to_string(&Save {
        as_ver: env!("CARGO_PKG_VERSION").to_string(),
        game_ver: match &info.config {
            Some(c) => c.version.to_string(),
            None => panic!("Config file not initialized"),
        },
        script: info.script_name().to_string(),
        pointer: info.pointer,
        flags: info.flags.clone(),
        variables: info.variables.clone(),
        screentext: info.screentext.to_string(),
    })
    .unwrap();
    info
        //TODO: multisave
        .load_file("save.ad2", "w", FileType::Save)?
        .write(save.as_bytes())?;
    Ok(())
}
