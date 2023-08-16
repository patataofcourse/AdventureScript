use crate::{
    core::{
        error::{ASOtherError, ASSyntaxError, ASVarError},
        ASType, ASVariable, AdventureIO, FileType,
    },
    formats::{config, config::Config, save},
    modules::ObjSpec,
};
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

pub struct GameInfo {
    pub io: AdventureIO,
    pub root_dir: PathBuf,
    pub script_name: String,
    script: Vec<String>,
    pub pointer: i64,
    pub quitting: bool,
    pub flags: HashMap<String, ASVariable>,
    pub variables: HashMap<String, ASVariable>,
    pub config: Option<Config>,
    pub local: bool,
    pub debug: bool,
    pub allow_save: bool,
    pub screentext: String,
    pub objects: Vec<ObjSpec>,
    pub mod_globals: HashMap<String, ASVariable>, //TODO: add to save
}

impl GameInfo {
    pub fn create(root_dir: PathBuf, io: AdventureIO, local: bool, debug: bool) -> GameInfo {
        GameInfo {
            io,
            root_dir,
            script_name: "start".to_string(),
            script: Vec::<String>::new(),
            pointer: 0,
            quitting: false,
            flags: HashMap::<String, ASVariable>::new(),
            variables: HashMap::<String, ASVariable>::new(),
            config: None,
            local,
            debug,
            allow_save: true,
            screentext: String::new(),
            objects: vec![],
            mod_globals: HashMap::new(),
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
    pub fn pointer(&self) -> i64 {
        self.pointer + 1
    }
    pub fn root_dir(&self) -> &PathBuf {
        &self.root_dir
    }
    pub fn get_line(&self) -> anyhow::Result<&str> {
        //obtains the current line of the script
        match self.script.get(self.pointer as usize) {
            Some(c) => Ok(c.trim_end()),
            None => Err(ASSyntaxError::EndOfScript {})?,
        }
    }
    /// obtains the line of the script at a certain position, if it exists
    pub fn line_at(&self, pointer: i64) -> Option<&str> {
        match self.script.get(pointer as usize) {
            Some(c) => Some(c.as_str()),
            None => None,
        }
    }

    // IO stuff
    pub fn show(&mut self, text: &str) -> anyhow::Result<()> {
        self.io.show(text)?;
        self.screentext += &format!("{}\n", text);
        Ok(())
    }
    pub fn wait(&self) -> anyhow::Result<()> {
        self.io.wait()
    }
    pub fn error(&self, text: String) {
        self.io.error(text)
    }
    pub fn warn(&self, text: String) {
        self.io.warn(text)
    }
    pub fn load_file(&self, filename: &str, mode: &str, ftype: FileType) -> anyhow::Result<File> {
        self.io.load_file(self, filename, mode, ftype)
    }
    //TODO: complete

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

        let mut instances = Vec::<i64>::new(); //lines where there's been a match
        for (c, line) in self.script.iter().enumerate() {
            if line.trim() == format!("{{{}}}", lname) {
                instances.push(c as i64);
            }
        }
        match instances.len() {
            0 => Err(ASSyntaxError::NonExistentLabel(lname.to_string()))?,
            1 => {
                self.pointer = *instances.first().unwrap();
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
                    if self.flags.get(name).is_none() {
                        self.flags.insert(name.to_string(), ASVariable::Bool(false));
                    }
                    self.flags.get(name).unwrap()
                } else {
                    //TODO: modules / module globals
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

    pub fn get_var_mut(&mut self, var: &ASVariable) -> anyhow::Result<&mut ASVariable> {
        Ok(match var {
            ASVariable::VarRef { name, flag } => {
                if *flag {
                    if self.flags.get(name).is_none() {
                        self.flags.insert(name.to_string(), ASVariable::Bool(false));
                    }
                    self.flags.get_mut(name).unwrap()
                } else {
                    match self.variables.get_mut(name) {
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
            } else if self.variables.remove(name).is_none() {
                Err(ASVarError::VarNotFound(name.to_string()))?
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
            self.io.show(text)?;
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
                        save::save(self)?;
                    }
                    return Ok(0);
                }
                "r" => {
                    if self.allow_save {
                        save::restore(self)?;
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
                self.screentext = String::new();
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
        let lines = file.split('\n');
        for line in lines {
            self.script.push(line.to_string());
        }
        self.pointer = 0;
        Ok(())
    }

    pub(crate) fn show_screentext(&self) -> anyhow::Result<()> {
        self.io.show(&self.screentext)
    }

    pub(crate) fn add_module(
        &mut self,
        objects: Vec<ObjSpec>,
        globals: HashMap<String, ASType>,
        name: &str,
    ) {
        for object in objects {
            self.objects.push(object.adapt_for_module(name));
        }
        for (gname, global) in globals {
            self.mod_globals
                .insert(format!("{}.{}", name, gname), global.default_for_type());
        }
    }

    pub fn get_object(&self, spec: &str) -> Option<ObjSpec> {
        for object in &self.objects {
            if object.name == spec {
                return Some(object.clone());
            }
        }
        None
    }
}
