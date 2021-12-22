use crate::core::{
    error::{ASFileError, FileErrors},
    FileType, GameInfo,
};
use semver::Version;
use serde_derive::Deserialize;
use std::{io::Read, path::PathBuf};

// Needed, since Version can't be serialized/deserialized
#[derive(Deserialize, Debug)]
struct UnparsedVerConfig {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub icon: Option<PathBuf>,
    pub module: Option<Vec<Module>>,
}

impl UnparsedVerConfig {
    pub fn parse_ver(self) -> anyhow::Result<Config> {
        Ok(Config {
            name: self.name,
            description: self.description,
            version: match Version::parse(&self.version) {
                Ok(c) => c,
                Err(e) => Err(ASFileError::from(
                    "info.toml",
                    "r",
                    FileErrors::ConfigLoadError(format!("Error parsing field 'version': {}", e)),
                ))?,
            },
            icon: self.icon,
            module: self.module,
        })
    }
}

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub description: Option<String>,
    pub version: Version,
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
    let config: UnparsedVerConfig = match toml::from_str(&file) {
        Ok(c) => c,
        Err(e) => Err(ASFileError::from(
            "info.toml",
            "r",
            FileErrors::ConfigLoadError(e.to_string()),
        ))?,
    };
    config.parse_ver()
}
