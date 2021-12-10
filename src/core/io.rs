use crate::core::{
    error::{ASFileError, ASOtherError, FileErrors},
    info::GameInfo,
};
use anyhow;
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

fn show_(text: &str) -> anyhow::Result<()> {
    println!("{}", text);
    Ok(())
}

fn wait_() -> anyhow::Result<()> {
    stdin().read(&mut [0])?;
    Ok(())
}

fn input_() -> anyhow::Result<String> {
    print!("> ");
    stdout().flush()?;
    let mut result = String::new();
    stdin().read_line(&mut result)?;
    Ok(result)
}

pub enum FileType {
    Script,
    Save,
    CustomDir(String),
    Other,
}

fn pc_save_location(info: &GameInfo) -> anyhow::Result<std::path::PathBuf> {
    //TODO other platforms?
    return match dirs::data_local_dir() {
        Some(c) => {
            let mut c = c;
            c.extend(&PathBuf::from(info.root_dir()));
            Ok(c)
        }
        None => Err(ASOtherError::UnsupportedPlatform)?,
    };
}

fn load_file_(
    info: &GameInfo,
    filename: &str,
    mode: &str,
    ftype: FileType,
) -> anyhow::Result<File> {
    let folder = match ftype {
        FileType::Script => PathBuf::from("script"),
        FileType::CustomDir(c) => PathBuf::from(c),
        FileType::Save => {
            //TODO: make save directory if it doesn't exist
            if info.local {
                PathBuf::from("save")
            } else {
                pc_save_location(info)?
            }
        }
        FileType::Other => PathBuf::new(),
    };

    let mut fname = info.root_dir().clone();
    fname.push(&folder);
    fname.push(PathBuf::from(filename));

    //this manages std::io errors
    let return_errors = |i: std::io::Result<File>| -> anyhow::Result<File> {
        match i {
            Ok(c) => Ok(c),
            Err(e) => {
                use std::io::ErrorKind as EK;
                match e.kind() {
                    EK::NotFound => Err(ASFileError::from(
                        &fname.to_string_lossy(),
                        mode,
                        FileErrors::NotFound,
                    ))?,
                    EK::PermissionDenied => Err(ASFileError::from(
                        &fname.to_string_lossy(),
                        mode,
                        FileErrors::MissingPermissions,
                    ))?,

                    _ => Err(e)?,
                }
            }
        }
    };

    Ok(match mode {
        "r" => return_errors(File::open(&fname))?,
        "w" => return_errors(File::create(&fname))?,
        _ => Err(ASFileError::from(
            &fname.to_string_lossy(),
            mode,
            FileErrors::InvalidMode(mode.to_string()),
        ))?,
    })
}

fn error_(text: String) {
    eprintln!("{}", text)
}

fn warn_(text: String) {
    eprintln!("WARNING: {}", text)
}

pub struct AdventureIO {
    show: fn(&str) -> anyhow::Result<()>,
    wait: fn() -> anyhow::Result<()>,
    input: fn() -> anyhow::Result<String>,
    load_file: fn(&GameInfo, &str, &str, FileType) -> anyhow::Result<File>,
    error: fn(String),
    warn: fn(String),
}

impl AdventureIO {
    pub fn show(&self, text: &str) -> anyhow::Result<()> {
        (self.show)(text)
    }
    pub fn wait(&self) -> anyhow::Result<()> {
        (self.wait)()
    }
    pub fn input(&self) -> anyhow::Result<String> {
        (self.input)()
    }
    pub fn load_file(
        &self,
        info: &GameInfo,
        filename: &str,
        mode: &str,
        ftype: FileType,
    ) -> anyhow::Result<File> {
        (self.load_file)(info, filename, mode, ftype)
    }
    pub fn error(&self, text: String) {
        (self.error)(text)
    }
    pub fn warn(&self, text: String) {
        (self.warn)(text)
    }

    pub fn default_with(
        show: Option<fn(&str) -> anyhow::Result<()>>,
        wait: Option<fn() -> anyhow::Result<()>>,
        input: Option<fn() -> anyhow::Result<String>>,
        load_file: Option<fn(&GameInfo, &str, &str, FileType) -> anyhow::Result<File>>,
        error: Option<fn(String)>,
        warn: Option<fn(String)>,
    ) -> Self {
        Self {
            show: show.unwrap_or(show_),
            wait: wait.unwrap_or(wait_),
            input: input.unwrap_or(input_),
            load_file: load_file.unwrap_or(load_file_),
            error: error.unwrap_or(error_),
            warn: warn.unwrap_or(warn_),
        }
    }
}

impl Default for AdventureIO {
    fn default() -> Self {
        Self {
            show: show_,
            wait: wait_,
            input: input_,
            load_file: load_file_,
            error: error_,
            warn: warn_,
        }
    }
}
