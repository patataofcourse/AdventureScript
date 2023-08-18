use std::collections::HashMap;

use never_say_never::Never;
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    spanned::Spanned,
    Expr, ExprLit, ExprTuple, Ident, Lit, LitBool, Meta, MetaList, Token,
};

macro_rules! error {
    ($span:expr => $str:literal $(, $arg:expr)*$(,)?) => {
        venial::Error::new_at_span($span, format!($str, $($arg),*)).to_compile_error().into()
    };
    ($str:literal $(, $arg:expr)*$(,)?) => {
        venial::Error::new(format!($str, $($arg),*)).to_compile_error().into()
    }
}

#[allow(unused)]
macro_rules! todo {
    ($str:literal $(, $arg:expr)*$(,)?) => {
        venial::Error::new(format!(concat!("not yet implemented: ", $str), $($arg),*)).to_compile_error().into()

    };
    () => {
        venial::Error::new("not yet implemented").to_compile_error().into()
    }
}

#[derive(Default)]
pub struct AttrArgs {
    pub named: Vec<String>,
    pub value: HashMap<String, Expr>,
}

impl Parse for AttrArgs {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut out = Self {
            ..Default::default()
        };

        loop {
            if input.is_empty() {
                break;
            }

            let name_ident = input.parse::<Ident>()?;
            let name = name_ident.to_string();

            if out.named.iter().any(|c| *c == name) || out.value.iter().any(|(k, _)| **k == name) {
                Err(syn::Error::new(
                    name_ident.span(),
                    "duplicate attribute argument",
                ))?
            }

            if let Ok(c) = input.parse::<Token!(=)>() {
                let val = input.parse::<Expr>()?;
                out.value.insert(name, val);
            } else {
                out.named.push(name)
            }

            if input.is_empty() {
                break;
            }

            input.parse::<Token!(,)>()?;
        }
        Ok(out)
    }
}
