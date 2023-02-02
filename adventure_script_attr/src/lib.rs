extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, FnArg, ItemFn, Type};

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    // get signature and adapt to an AS signature
    let mut args: Vec<()> = vec![];
    for argument in input.sig.inputs.iter() {
        match argument {
            FnArg::Receiver(_) => {
                return quote_spanned!(argument.span()=>
                    compile_error!("command macro: commands cannot have 'self' as an argument");
                )
                .into();
            }
            FnArg::Typed(c) => match c.ty.as_ref() {
                Type::Infer(ty) => {
                    return quote_spanned!(ty.span()=>
                        compile_error!("command macro: cannot infer types");
                    )
                    .into();
                }
                Type::Paren(ty) => {
                    //TODO: rerun the match
                    return quote_spanned!(ty.span() => compile_error!("not yet implemented");)
                        .into();
                }
                // explicit type description
                Type::Path(ty) => {
                    //TODO: try to resolve type
                    return quote_spanned!(ty.span() => compile_error!("not yet implemented");)
                        .into();
                }
                c => {
                    todo!("{:?}", c);
                }
            },
        }
    }
    todo!();
}
