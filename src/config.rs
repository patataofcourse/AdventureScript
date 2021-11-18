use crate::{info::GameInfo, io::FileType};
use serde_derive::Deserialize;
use std::{io::Read, path::PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub icon: Option<PathBuf>,
    pub module: Option<Vec<Module>>,
    pub io: Option<IO>,
}

#[derive(Deserialize)]
pub struct Module {
    pub name: String,
    pub file: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct IO {
    pub name: String,
    pub file: Option<PathBuf>,
}

pub fn load_config(info: GameInfo) -> anyhow::Result<Config> {
    let mut file = String::from("");
    info.io()
        .load_file(&info, "info.toml", "r", FileType::Other)?
        .read_to_string(&mut file)?;
    let config: Config = toml::from_str(&file)?;
    Ok(config)
}
