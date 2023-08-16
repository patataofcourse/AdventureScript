extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, AttributeArgs};

#[macro_use]
mod util;

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    let input = match venial::parse_declaration(input) {
        Ok(c) => c,
        Err(e) => return e.to_compile_error().into(),
    };

    let func = match input.as_function() {
        Some(c) => c.clone(),
        None => return error!("only functions are supported by #[command]"),
    };

    let args = parse_macro_input!(args as AttributeArgs);
    let args = match util::manage_attr_args(args) {
        Ok(c) => c,
        Err(e) => return e,
    };

    todo!()
}
