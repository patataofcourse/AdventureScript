#[macro_export]
//TODO: proc macro?
macro_rules! command {
    //TODO: replace dexpr with a tokentree
    ($name:ident $([deprecated=$is_depr:literal])? $((
        $(
            $(!$pname:ident:$ptype:ident),+,
        )?
        $(
            $($dname:ident:$dtype:ident=$dexpr:expr),+,
        )?
    ))? => $function: expr) => {
        $crate::core::Command::new (
            stringify!($name).to_string(),
            $function,
            vec![
                $(
                    $($(stringify!($pname).to_string()),+,)?
                    $($(stringify!($dname).to_string()),+,)?
                )?
            ],
            ::std::collections::HashMap::<String, $crate::core::ASType>::from_iter([
                $(
                    $($((stringify!($pname).to_string(),
                    $crate::core::ASType::$ptype,)),+,)?
                    $($((stringify!($dname).to_string(),
                    $crate::core::ASType::$dtype,)),+,)?
                )?
            ]),
            std::collections::HashMap::<String, $crate::core::ASVariable>::from_iter([
                $(
                    $($((stringify!($dname).to_string(),
                    $crate::core::ASVariable::$dtype($dexpr),)),+,)?
                )?
            ]),
            $($is_depr && !)? false,
            )
    };
}

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
