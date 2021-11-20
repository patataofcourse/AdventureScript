#[macro_export]
macro_rules! command {
    ($name:literal $((
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
        $crate::Command {
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
                    $crate::ASType::$ptype,)),+,)?
                    $($(($dname.to_string(),
                    $crate::ASType::$dtype,)),+,)?
                    $($(($kname.to_string(),
                    $crate::ASType::$ktype,)),+,)?
                )?
            ]),
            default_values: std::collections::HashMap::<String, ASVariable>::from_iter([
                $(
                    $($(($dname.to_string(),
                    $crate::ASVariable::$dtype($dexpr),)),+,)?
                    $($(($kname.to_string(),
                    $crate::ASVariable::$ktype($kexpr),)),+,)?
                )?
            ]),
        }
    };
}

#[macro_export]
macro_rules! get_var {
    ($map:ident->$vname:literal:$vtype:ident or None) => {{
        match $map.get($vname).expect("Non-existent argument on get_var") {
            $crate::ASVariable::$vtype(c) => Some(c),
            $crate::ASVariable::None => None,
            _ => panic!("Wrong type on get_var"),
        }
    }};
    ($map:ident->$vname:literal:$vtype:ident) => {{
        let var = match $map.get($vname).expect("Non-existent argument on get_var") {
            $crate::ASVariable::$vtype(c) => Some(c),
            _ => panic!("Wrong type on get_var"),
        };
        match var {
            Some(c) => c,
            _ => panic!("get_var: Got a None value from a variable that shouldn't be None"),
        }
    }};
}
