//TODO: remove this when i make a proc-macro for methods
#[macro_export]
macro_rules! unwrap_var {
    ($map:ident->$vname:expr; Option<$vtype:ident>) => {{
        match $map.get($vname) {
            Some($crate::core::ASVariable::$vtype(c)) => Ok(Some(c)),
            None | Some($crate::core::ASVariable::None) => Ok(None),
            _ => Err($crate::core::error::ASOtherError::DevErr(
                "Wrong type on unwrap_var".to_string(),
            )),
        }
    }};
    ($map:ident->$vname:expr;$vtype:ident) => {{
        match $map.get($vname) {
            Some($crate::core::ASVariable::$vtype(c)) => Ok(c),
            None | Some($crate::core::ASVariable::None) => {
                Err($crate::core::error::ASOtherError::DevErr(
                    "unwrap_var: Got a None value from a variable that shouldn't be None"
                        .to_string(),
                ))
            }
            _ => Err($crate::core::error::ASOtherError::DevErr(
                "Wrong type on unwrap_var".to_string(),
            )),
        }
    }};
}
