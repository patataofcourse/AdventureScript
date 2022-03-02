#[macro_export]
macro_rules! command {
    //TODO: replace dexpr/kexpr with a tokentree
    //TODO: maybe remove kwarg functionality and keep only posargs? it's not like an *args functionality is done
    ($name:ident $([deprecated=$is_depr:literal])? $((
        $(
            $(!$pname:ident:$ptype:ident),+,
        )?
        $(
            $($dname:ident:$dtype:ident=$dexpr:expr),+,
        )?
        $(
            *,
            $($kname:ident:$ktype:ident=$kexpr:expr),+,
        )?
    ))? => |$info:ident, $kwargs:ident| $fnbody: tt) => {
        $crate::core::Command::new (
            stringify!($name).to_string(),
            |$info, $kwargs| $fnbody,
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
                    $($((stringify!($kname).to_string(),
                    $crate:::core:ASType::$ktype,)),+,)?
                )?
            ]),
            std::collections::HashMap::<String, $crate::core::ASVariable>::from_iter([
                $(
                    $($((stringify!($dname).to_string(),
                    $crate::core::ASVariable::$dtype($dexpr),)),+,)?
                    $($((stringify!($kname).to_string(),
                    $crate::core::ASVariable::$ktype($kexpr),)),+,)?
                )?
            ]),
            $(if $is_depr {true} else {false} && !)? false,
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
