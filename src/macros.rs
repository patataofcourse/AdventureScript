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
        $crate::core::Command {
            name: stringify!($name).to_string(),
            func: |$info, $kwargs| $fnbody,
            args_to_kwargs: vec![
                $(
                    $($(stringify!($pname).to_string()),+,)?
                    $($(stringify!($dname).to_string()),+,)?
                )?
            ],
            accepted_kwargs: ::std::collections::HashMap::<String, ASType>::from_iter([
                $(
                    $($((stringify!($pname).to_string(),
                    $crate::core::ASType::$ptype,)),+,)?
                    $($((stringify!($dname).to_string(),
                    $crate::core::ASType::$dtype,)),+,)?
                    $($((stringify!($kname).to_string(),
                    $crate:::core:ASType::$ktype,)),+,)?
                )?
            ]),
            default_values: std::collections::HashMap::<String, ASVariable>::from_iter([
                $(
                    $($((stringify!($dname).to_string(),
                    $crate::core::ASVariable::$dtype($dexpr),)),+,)?
                    $($((stringify!($kname).to_string(),
                    $crate::core::ASVariable::$ktype($kexpr),)),+,)?
                )?
            ]),
            deprecated: $(if $is_depr {true} else {false} && !)? false,
        }
    };
}

#[macro_export]
macro_rules! unwrap_var {
    ($map:ident->$vname:expr; Option<$vtype:ident>) => {{
        match $map.get($vname) {
            Some($crate::ASVariable::$vtype(c)) => Some(c),
            None | Some($crate::ASVariable::None) => None,
            _ => Err($crate::error::ASOtherError::DevErr(
                "Wrong type on unwrap_var".to_string(),
            ))?,
        }
    }};
    ($map:ident->$vname:expr;$vtype:ident) => {{
        let var = match $map
            .get($vname)
            .expect("Non-existent argument on unwrap_var")
        {
            $crate::core::ASVariable::$vtype(c) => Some(c),
            _ => Err($crate::core::error::ASOtherError::DevErr(
                "Wrong type on unwrap_var".to_string(),
            ))?,
        };
        match var {
            Some(c) => c,
            None => Err($crate::core::error::ASOtherError::DevErr(
                "unwrap_var: Got a None value from a variable that shouldn't be None".to_string(),
            ))?,
        }
    }};
}
