use std::collections::HashMap;

use never_say_never::Never;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, Lit};

macro_rules! error {
    ($span:expr => $str:literal $(, $arg:expr)*$(,)?) => {
        venial::Error::new_at_span($span, format!($str, $($arg),*)).to_compile_error().into()
    };
    ($str:literal $(, $arg:expr)*$(,)?) => {
        venial::Error::new(format!($str, $($arg),*)).to_compile_error().into()
    }
}

macro_rules! todo {
    ($str:literal $(, $arg:expr)*$(,)?) => {
        #[must_use]
        venial::Error::new(format!(concat!("not yet implemented: ", $str), $($arg),*)).to_compile_error().into()
    };
    () => {
        venial::Error::new("not yet implemented").to_compile_error().into()
    }
}

pub fn manage_attr_args(args: AttributeArgs) -> Result<HashMap<String, Lit>, TokenStream> {

    let mut out = HashMap::new();

    let key_val_error = |span| -> Result<Never, TokenStream> {
        Err(error!(span => "arguments for the attribute must be of `key = value` format"))
    };

    for arg in args {
        match arg {
            syn::NestedMeta::Meta(c) => match c {
                syn::Meta::Path(c) => key_val_error(c.span())?,
                syn::Meta::List(c) => key_val_error(c.span())?,
                syn::Meta::NameValue(c) => out.insert(c.path.to_token_stream().to_string(), c.lit),
            },
            syn::NestedMeta::Lit(c) => key_val_error(c.span())?,
        };
    }

    Ok(out)
}
