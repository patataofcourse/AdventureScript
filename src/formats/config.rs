use crate::core::{
    error::{ASFileError, FileErrors},
    FileType, GameInfo,
};
use serde_derive::Deserialize;
use std::{io::Read, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub description: Option<String>,
    //TODO: make the version a SemVer
    pub version: String,
    pub icon: Option<PathBuf>,
    pub module: Option<Vec<Module>>,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub file: Option<PathBuf>,
}

pub fn load_config(info: &GameInfo) -> anyhow::Result<Config> {
    let mut file = String::from("");
    info.load_file("info.toml", "r", FileType::Other)?
        .read_to_string(&mut file)?;
    let config: Config = match toml::from_str(&file) {
        Ok(c) => c,
        Err(e) => Err(ASFileError::from(
            "info.toml",
            "r",
            FileErrors::ConfigLoadError(e.to_string()),
        ))?,
    };
    Ok(config)
}
