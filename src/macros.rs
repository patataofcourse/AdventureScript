#[macro_export]
macro_rules! command {
    //TODO: replace dexpr/kexpr with a tokentree
    //TODO: maybe remove kwarg functionality and keep only posargs? it's not like an *args functionality is done
    ($([deprecated=$is_depr:literal])? $name:literal $((
        $(
            $(!$pname:literal:$ptype:ident),+,
        )?
        $(
            $($dname:literal:$dtype:ident=$dexpr:expr),+,
        )?
        $(
            *,
            $($kname:literal:$ktype:ident=$kexpr:expr),+,
        )?
    ))? => |$cmd:ident, $info:ident, $kwargs:ident| $fnbody: tt) => {
        $crate::core::Command {
            name: $name.to_string(),
            func: |$cmd, $info, $kwargs| $fnbody,
            args_to_kwargs: vec![
                $(
                    $($($pname.to_string()),+,)?
                    $($($dname.to_string()),+,)?
                )?
            ],
            accepted_kwargs: ::std::collections::HashMap::<String, ASType>::from_iter([
                $(
                    $($(($pname.to_string(),
                    $crate::core::ASType::$ptype,)),+,)?
                    $($(($dname.to_string(),
                    $crate::core::ASType::$dtype,)),+,)?
                    $($(($kname.to_string(),
                    $crate:::core:ASType::$ktype,)),+,)?
                )?
            ]),
            default_values: std::collections::HashMap::<String, ASVariable>::from_iter([
                $(
                    $($(($dname.to_string(),
                    $crate::core::ASVariable::$dtype($dexpr),)),+,)?
                    $($(($kname.to_string(),
                    $crate::core::ASVariable::$ktype($kexpr),)),+,)?
                )?
            ]),
            deprecated: $(if $is_depr {true} else {false} && !)? false,
        }
    };
}

#[macro_export]
macro_rules! get_var {
    ($map:ident->$vname:expr;$vtype:ident.or_none) => {{
        match $map.get($vname).expect("Non-existent argument on get_var") {
            $crate::ASVariable::$vtype(c) => Some(c),
            $crate::ASVariable::None => None,
            _ => Err($crate::error::ASOtherError::DevErr(
                "Wrong type on get_var".to_string(),
            ))?,
        }
    }};
    ($map:ident->$vname:expr;$vtype:ident) => {{
        let var = match $map.get($vname).expect("Non-existent argument on get_var") {
            $crate::core::ASVariable::$vtype(c) => Some(c),
            _ => Err($crate::core::error::ASOtherError::DevErr(
                "Wrong type on get_var".to_string(),
            ))?,
        };
        match var {
            Some(c) => c,
            None => Err($crate::core::error::ASOtherError::DevErr(
                "get_var: Got a None value from a variable that shouldn't be None".to_string(),
            ))?,
        }
    }};
}
