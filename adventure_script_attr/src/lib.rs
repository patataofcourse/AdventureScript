extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, Ident, Lit, Path};

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
    let fn_name = func.name.clone();

    let args = parse_macro_input!(args as AttributeArgs);
    let args = match util::manage_attr_args(args) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let name = match args.get("name") {
        Some(c) => c.to_token_stream(),
        None => {
            let name = fn_name.to_string();
            quote! {#name}
        }
    };
    let crate_path = match args.get("crate_path") {
        Some(Lit::Str(c)) => match c.parse_with(Path::parse_mod_style) {
            Ok(c) => c,
            Err(e) => return e.to_compile_error().into(),
        },
        Some(c) => return error!(c.span() => "must be a string containing a path, eg. `\"as\"`"),
        None => Ident::new("adventure_script", input.span()).into(),
    };
    let deprecated = args.get("deprecated").is_some();

    //todo!();

    quote! {
        //TODO: remove this when done
        #[allow(unreachable_code)]
        //TODO: figure out the result type
        pub fn #fn_name () -> #crate_path::Result<#crate_path::core::Command> {
            #crate_path::core::Command {
                name: String::from(#name),
                func: todo!(),
                args: todo!(),
                deprecated: #deprecated,
            }
        }
    }
    .into()
}
