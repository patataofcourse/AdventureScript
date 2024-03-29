use crate::core::{
    error::{ASFileError, FileErrors},
    ASVariable, FileType, GameInfo,
};
use semver::{Version, VersionReq};
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Save {
    pub as_ver: String,
    pub game_ver: String,
    pub script: PathBuf,
    //TODO: save position as a label??
    pub pointer: i64,
    pub flags: HashMap<String, ASVariable>,
    pub variables: HashMap<String, ASVariable>,
    pub screentext: String,
}

pub fn restore(info: &mut GameInfo) -> anyhow::Result<bool> {
    //TODO: multisave
    let save_path = "save.ad2";

    let mut file = String::from("");
    match info.load_file(save_path, "r", FileType::Save) {
        Ok(c) => c,
        Err(e) => match e.downcast_ref::<ASFileError>() {
            Some(ASFileError {
                details: FileErrors::NotFound,
                ..
            }) => {
                info.show("No save available")?;
                return Ok(false);
            }
            _ => Err(e)?,
        },
    }
    .read_to_string(&mut file)?;
    let save: Save = match serde_json::from_str(&file) {
        Ok(c) => c,
        Err(e) => Err(ASFileError::from(
            &format!("save/{}", save_path),
            "r",
            FileErrors::SaveLoadError(e.to_string()),
        ))?,
    };

    let ver = match Version::parse(&save.as_ver) {
        Ok(c) => c,
        Err(e) => Err(ASFileError::from(
            &format!("save/{}", save_path),
            "r",
            FileErrors::SaveLoadError(e.to_string()),
        ))?,
    };

    if !VersionReq::parse(&format!(">= 2.0.0-alpha.1")) //TODO: update on betas
        .unwrap()
        .matches(&ver)
    {
        Err(ASFileError::from(
            &format!("save/{}", save_path),
            "r",
            FileErrors::SaveNotCompatible(ver.to_string()),
        ))?
    }

    if !VersionReq::parse(&format!("<= {}", crate::get_version()))
        .unwrap()
        .matches(&ver)
    {
        Err(ASFileError::from(
            &format!("save/{}", save_path),
            "r",
            FileErrors::SaveTooNew(ver.to_string()),
        ))?
    }

    info.load_script(Some(
        match &save.script.as_os_str().to_os_string().into_string() {
            Ok(c) => c,
            Err(_) => Err(ASFileError::from(
                &format!("save/{}", save_path),
                "r",
                FileErrors::SaveLoadError(
                    "Path to script invalid - make sure it's UTF-8 compatible".to_string(),
                ),
            ))?,
        },
    ))?;

    info.show("Restored save\n")?;

    info.pointer = save.pointer;
    info.flags = save.flags;
    info.variables = save.variables;
    info.screentext = save.screentext;
    info.show_screentext()?;

    Ok(true)
}

pub fn save(info: &mut GameInfo) -> anyhow::Result<()> {
    //TODO: add multisave
    let save_path = "save.ad2";

    let screentext = info.screentext.to_string();
    let save = serde_json::to_string(&Save {
        as_ver: env!("CARGO_PKG_VERSION").to_string(),
        game_ver: match &info.config {
            Some(c) => c.version.to_string(),
            None => panic!("Config file not initialized"),
        },
        script: PathBuf::from(info.script_name()),
        pointer: info.pointer,
        flags: info.flags.clone(),
        variables: info.variables.clone(),
        screentext: screentext.clone(),
    })
    .unwrap();
    info.load_file(save_path, "w", FileType::Save)?
        .write(save.as_bytes())?;

    info.show("Saved")?;
    info.screentext = screentext; // so the "Saved." doesn't get added to the screentext
    Ok(())
}
