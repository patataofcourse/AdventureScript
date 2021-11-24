use crate::{
    config,
    config::Config,
    error::{ASOtherError, ASSyntaxError, ASVarError},
    io::{AdventureIO, FileType},
    variables::ASVariable,
};
use std::{collections::HashMap, io::Read, path::PathBuf};

pub struct GameInfo {
    io: AdventureIO,
    root_dir: PathBuf,
    script_name: String,
    script: Vec<String>,
    pub pointer: i32,
    pub quitting: bool,
    pub flags: HashMap<String, ASVariable>,
    pub variables: HashMap<String, ASVariable>,
    config: Option<Config>,
    pub local: bool,
    pub allow_save: bool,
}

impl GameInfo {
    pub fn create(root_dir: PathBuf, io: AdventureIO, local: bool) -> GameInfo {
        GameInfo {
            io: io,
            root_dir: root_dir,
            script_name: String::from("start"),
            script: Vec::<String>::new(),
            pointer: 0,
            quitting: false,
            flags: HashMap::<String, ASVariable>::new(),
            variables: HashMap::<String, ASVariable>::new(),
            config: None,
            local,
            allow_save: true,
        }
    }

    pub fn load_config(&mut self) -> anyhow::Result<()> {
        let config = config::load_config(self)?;
        self.config = Some(config);
        Ok(())
    }

    // getting some of its items
    pub fn script_name(&self) -> &str {
        &self.script_name
    }
    pub fn pointer(&self) -> i32 {
        self.pointer + 1
    }
    pub fn root_dir(&self) -> &PathBuf {
        &self.root_dir
    }
    pub fn get_line(&self) -> anyhow::Result<String> {
        //obtains the current line of the script
        match self.script.get(self.pointer as usize) {
            Some(c) => Ok(c.trim_end().to_string()),
            None => Err(ASSyntaxError::EndOfScript {})?,
        }
    }
    pub fn io(&self) -> &AdventureIO {
        &self.io
    }

    pub fn goto_label(&mut self, var: &ASVariable) -> anyhow::Result<()> {
        let lname = match var {
            ASVariable::Label(c) => match c {
                None => return Ok(()),
                Some(c) => c,
            },
            _ => Err(ASOtherError::DevErr(
                "Used goto_label function with a non-label ASVariable".to_string(),
            ))?,
        };

        let mut c = 0; //loop counter
        let mut instances = Vec::<i32>::new(); //lines where there's been a match
        for line in &self.script {
            if line.trim() == format!("{{{}}}", lname) {
                instances.push(c);
            }
            c += 1;
        }
        match instances.len() {
            0 => Err(ASSyntaxError::NonExistentLabel(lname.to_string()))?,
            1 => {
                self.pointer = *instances.get(0).unwrap();
                Ok(())
            }
            _ => Err(ASSyntaxError::RepeatedLabel(lname.to_string(), instances))?,
        }
    }

    pub fn next_line(&mut self) {
        self.pointer += 1;
    }

    pub fn quit(&mut self) {
        self.quitting = true;
    }

    pub fn get_var(&mut self, var: &ASVariable) -> anyhow::Result<&ASVariable> {
        Ok(match var {
            ASVariable::VarRef { name, flag } => {
                if *flag {
                    if let None = self.flags.get(name) {
                        self.flags.insert(name.to_string(), ASVariable::Bool(false));
                    }
                    self.flags.get(name).unwrap()
                } else {
                    match self.variables.get(name) {
                        Some(c) => c,
                        None => Err(ASVarError::VarNotFound(name.to_string()))?,
                    }
                }
            }
            _ => Err(ASOtherError::DevErr(
                "Tried to get the variable value of a non-VarRef value".to_string(),
            ))?,
        })
    }

    pub fn set_var(&mut self, var: &ASVariable, value: ASVariable) -> anyhow::Result<()> {
        if let ASVariable::VarRef { name, flag } = var {
            if *flag {
                if let ASVariable::Bool(_) = value {
                    self.flags.insert(name.to_string(), value);
                } else {
                    Err(ASVarError::FlagNotBool(name.to_string()))?;
                }
            } else {
                self.variables.insert(name.to_string(), value);
            }
        } else {
            Err(ASOtherError::DevErr(
                "Tried to set the variable value of a non-VarRef value".to_string(),
            ))?
        }
        Ok(())
    }

    pub fn del_var(&mut self, var: &ASVariable) -> anyhow::Result<()> {
        if let ASVariable::VarRef { name, flag } = var {
            if *flag {
                self.flags.remove(&name.to_string());
            } else {
                if let None = self.variables.remove(name) {
                    Err(ASVarError::VarNotFound(name.to_string()))?
                }
            }
        } else {
            Err(ASOtherError::DevErr(
                "Tried to delete the variable value of a non-VarRef value".to_string(),
            ))?
        }
        Ok(())
    }

    //TODO: customization of choice text formatting
    pub fn query(&mut self, text: &str, choices: Vec<&str>) -> anyhow::Result<u8> {
        if !text.is_empty() {
            self.io.show(&text)?;
        }
        let mut c = 1;
        for ch in &choices {
            self.io.show(&format!("{}. {}", c, ch))?;
            c += 1;
        }
        loop {
            let result = self.io.input()?;
            match result.trim() {
                "s" => {
                    if self.allow_save {
                        self.io.show("Would save here")?;
                    }
                    //return Ok(0);
                }
                "r" => {
                    if self.allow_save {
                        crate::save::restore(self)?;
                    }
                    return Ok(0);
                }
                "q" => {
                    self.quit();
                    return Ok(0);
                }
                _ => (),
            }
            let num_result: u8 = match result.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if (num_result as usize) <= choices.len() {
                return Ok(num_result);
            }
        }
    }

    pub fn load_script(&mut self, filename: Option<&str>) -> anyhow::Result<()> {
        let filename = match filename {
            Some(c) => {
                self.script_name = c.to_string();
                c
            }
            None => &self.script_name,
        };
        let mut file = String::from("");
        self.script = vec![];
        self.io
            .load_file(self, &format!("{}.as2", filename), "r", FileType::Script)?
            .read_to_string(&mut file)?;
        let lines = file.split("\n");
        for line in lines {
            self.script.push(line.to_string());
        }
        self.pointer = 0;
        Ok(())
    }
}
