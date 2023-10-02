use std::collections::HashMap;

use adventure_script_attr::command;

use crate::core::variables::Label;
//TODO: remove allow() when the commands have been ported
#[allow(unused_imports)]
use crate::{
    core::{
        error::{ASCmdError, ASGameError, CommandErrors},
        variables::is_as_var::IsASVar,
        ASType, ASVariable, GameInfo,
    },
    formats::save,
};

use super::CmdSet;

#[command(crate_path = "crate")]
fn wait(info: &mut GameInfo) -> anyhow::Result<()> {
    info.wait()
}

#[command(crate_path = "crate")]
fn choice(
    info: &mut GameInfo,
    text: String,
    #[wrap_to(1..=9)] choices: Vec<Vec<ASVariable>>,
) -> anyhow::Result<()> {
    if choices.is_empty() {
        Err(ASCmdError {
            command: "choice".to_string(),
            details: CommandErrors::ChoiceEmptyChoices,
        })?
    }

    let mut texts = Vec::<String>::new();
    let mut gotos = Vec::<Label>::new();
    // separate the choices into the vectors defined above
    for (c, choice) in choices.iter().enumerate() {
        let text = match choice.get(0) {
            Some(s) => match s {
                ASVariable::String(c) => c.to_string(),
                ASVariable::VarRef { name, flag } => {
                    match info.get_var(&ASVariable::VarRef {
                        name: name.to_string(),
                        flag: *flag,
                    })? {
                        ASVariable::String(c) => c.to_string(),
                        other => Err(ASCmdError {
                            command: "choice".to_string(),
                            details: CommandErrors::ChoiceWrongType {
                                choice: c,
                                number: 2,
                                given: other.get_type(),
                                asked: ASType::Bool,
                            },
                        })?,
                    }
                }
                other => Err(ASCmdError {
                    command: "choice".to_string(),
                    details: CommandErrors::ChoiceWrongType {
                        choice: c,
                        number: 0,
                        given: other.get_type(),
                        asked: ASType::String,
                    },
                })?,
            },
            None => break,
        };
        let goto = match choice.get(1) {
            Some(v) => match v {
                ASVariable::None => None.into(),
                ASVariable::Label(c) => c.clone(),
                _ => Err(ASCmdError {
                    command: "choice".to_string(),
                    details: CommandErrors::ChoiceWrongType {
                        choice: c,
                        number: 1,
                        given: v.get_type(),
                        asked: ASType::Label,
                    },
                })?,
            },
            None => Err(ASCmdError {
                command: "choice".to_string(),
                details: CommandErrors::ChoiceMissingRequired {
                    typ: ASType::Label,
                    choice: c,
                },
            })?,
        };
        let flag = match choice.get(2) {
            Some(l) => match l {
                ASVariable::Bool(c) => *c,
                ASVariable::VarRef { name, flag } => {
                    match info.get_var(&ASVariable::VarRef {
                        name: name.to_string(),
                        flag: *flag,
                    })? {
                        ASVariable::Bool(c) => *c,
                        other => Err(ASCmdError {
                            command: "choice".to_string(),
                            details: CommandErrors::ChoiceWrongType {
                                choice: c,
                                number: 2,
                                given: other.get_type(),
                                asked: ASType::Bool,
                            },
                        })?,
                    }
                }
                other => Err(ASCmdError {
                    command: "choice".to_string(),
                    details: CommandErrors::ChoiceWrongType {
                        choice: c,
                        number: 2,
                        given: other.get_type(),
                        asked: ASType::Bool,
                    },
                })?,
            },
            None => true,
        };
        if flag {
            texts.push(text);
            gotos.push(goto.clone());
        }
    }
    let mut text_refs: Vec<&str> = vec![];
    for t in &texts {
        text_refs.push(t);
    }
    let pick = info.query(&text, text_refs)?;
    if pick == 0 {
        // used in save/return/quit
        info.pointer -= 1;
        return Ok(());
    };
    info.goto_label(gotos.get((pick - 1) as usize).unwrap())?;
    Ok(())
}

#[command(crate_path = "crate")]
fn goto(info: &mut GameInfo, label: Label) -> anyhow::Result<()> {
    info.goto_label(&label)
}

#[command(crate_path = "crate")]
fn loadscript(info: &mut GameInfo, script: String) -> anyhow::Result<()> {
    info.load_script(Some(&script))
}

#[command(crate_path = "crate")]
fn ending(info: &mut GameInfo, name: String) -> anyhow::Result<()> {
    info.show(&format!("Ending: {}", name))?;
    info.quit();
    Ok(())
}

pub fn main_commands() -> anyhow::Result<CmdSet> {
    Ok(CmdSet {
        commands: vec![wait()?, choice()?, goto()?, loadscript()?],
        aliases: HashMap::from_iter([
            ("load".to_string(), "loadscript".to_string()),
            ("ld".to_string(), "loadscript".to_string()),
            ("w".to_string(), "wait".to_string()),
            ("go".to_string(), "goto".to_string()),
            ("ch".to_string(), "choice".to_string()),
            //("sv".to_string(), "save".to_string()),
            ("end".to_string(), "ending".to_string()),
            //("lose".to_string(), "gameover".to_string()),
        ]),
        modules: HashMap::new(),
    })
}

// pub fn main_ocmannds() -> CmdSet {
//     CmdSet::from(
//         vec![
//             command_old! {
//                 flag (!flag: VarRef, value: Bool = true, ) => |info, kwargs| {
//                     let flag = match kwargs.get("flag").unwrap() {
//                         //Make sure you're getting a flag, not a variable
//                         ASVariable::VarRef { name, .. } => ASVariable::VarRef {
//                             name: name.to_string(),
//                             flag: true,
//                         },
//                         _ => panic!(),
//                     };
//                     info.set_var(&flag, kwargs.get("value").unwrap().clone())
//                 }
//             },
//             command_old! {
//               set (!var: VarRef, !value: Any,) => |info, kwargs| {
//                     let mut val = kwargs.get("value").unwrap().clone();
//                     while val.get_type() == ASType::VarRef {
//                         val = info.get_var(&val)?.clone();
//                     }
//                     info.set_var(
//                         kwargs.get("var").unwrap(),
//                         val,
//                     )
//                 }
//             },
//             command_old! {
//                 add (!var: VarRef, !value: Any,) => |info, kwargs| {
//                     let var = kwargs.get("var").unwrap();
//                     let val = info.get_var(var)?.clone();
//                     info.set_var(var, (val + kwargs.get("value").unwrap().clone())?)
//                 }
//             },
//             command_old! {
//                 if (!condition: Bool, !gotrue: Label, !gofalse: Label, ) => |info, kwargs| {
//                     let condition = *unwrap_var!(kwargs -> "condition"; Bool)?;
//                     if condition {
//                         info.goto_label(kwargs.get("gotrue").unwrap())
//                     } else {
//                         info.goto_label(kwargs.get("gofalse").unwrap())
//                     }
//                 }
//             },
//             command_old! {
//                 error (!message: String, ) => |_info, kwargs| {
//                     let message = unwrap_var!(kwargs -> "message"; String)?.to_string();
//                     Err(ASGameError(message))?
//                 }
//             },
//             command_old! {
//                 save (!val: Bool, ) => |info, kwargs| {
//                     info.allow_save = *unwrap_var!(kwargs -> "val"; Bool)?;
//                     Ok(())
//                 }
//             },
//             command_old! {
//                 gameover => |info, _kwargs| {
//                     info.show("**GAME OVER**")?;
//                     let query = info.query("Start over from last save?", vec!("Yes","No"))?;
//                     if query == 1 {
//                         if !save::restore(info)? {
//                             info.quit();
//                         };
//                     } else {
//                         info.quit();
//                     }
//                     Ok(())
//                 }
//             },
//             command_old! {
//                 del (!var: VarRef,) => |info, kwargs| {
//                     info.del_var(kwargs.get("var").unwrap())
//                 }
//             },
//             command_old! {
//                 switch (
//                     !check: Any,
//                     !values: List,
//                     !gotos: List,
//                     default: Label = None,
//                 ) => |info, kwargs| {
//                     let check = kwargs.get("check").unwrap();
//                     let values = unwrap_var!(kwargs -> "values"; List)?;
//                     let labels = unwrap_var!(kwargs -> "gotos"; List)?;
//                     let default = kwargs.get("default").unwrap();

//                     if values.len() != labels.len() {
//                         Err(ASCmdError {
//                             command: "switch".to_string(),
//                             details: CommandErrors::SwitchParams(values.len(), labels.len()),
//                         })?
//                     }

//                     for (c, value) in values.iter().enumerate() {
//                         let mut value = value.clone();
//                         while value.get_type() == ASType::VarRef {
//                             value = info.get_var(&value)?.clone();
//                         }

//                         if &value == check {
//                             let label = labels.get(c).unwrap();
//                             if label.get_type() != ASType::Label {
//                                 Err(ASCmdError {
//                                     command: "switch".to_string(),
//                                     details: CommandErrors::SwitchLabelType{
//                                         number: c,
//                                         given: label.get_type(),
//                                     }
//                                 })?
//                             }

//                             info.goto_label(label)?;
//                             return Ok(())
//                         }
//                     }
//                     info.goto_label(default)
//                 }
//             },
//             command_old! {
//                 append (!list: VarRef, !val: Any,) => |info, kwargs| {
//                     match info.get_var_mut(kwargs.get("list").unwrap())? {
//                         ASVariable::List(list) => {
//                             let val = kwargs.get("val").unwrap();
//                             list.push(val.clone());
//                             Ok(())
//                         }
//                         ASVariable::Map(map) => {
//                             todo!()
//                         }
//                         _ => Err(ASCmdError {
//                             command: "append".to_string(),
//                             details: CommandErrors::Generic {
//                                 details : "append can only be used with types List or Map".to_string(),
//                             },
//                         })?,
//                     }
//                 }
//             },
//         ],
//     )
// }
