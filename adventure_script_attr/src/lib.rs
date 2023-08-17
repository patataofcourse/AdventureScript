extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Expr, ExprLit, ExprRange, Ident, Lit, Path, ExprParen};
use venial::{FnParam, TyExpr};

#[macro_use]
mod util;

struct SignatureItem {
    pub name: Ident,
    pub ty: TyExpr,
    pub wrap_to: Option<ExprRange>,
}

// impl std::fmt::Debug for SignatureItem {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("SignatureItem")
//             .field("name", &self.name)
//             .field("ty", &self.ty)
//             .field("wrap_to", &self.wrap_to.to_token_stream().to_string())
//             .finish()
//     }
// }

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

    let args = match syn::parse::<util::AttrArgs>(args) {
        Ok(c) => c,
        Err(e) => return e.to_compile_error().into(),
    };
    let name = match args.value.get("name") {
        Some(c) => c.to_token_stream(),
        None => {
            let name = fn_name.to_string();
            quote! {#name}
        }
    };
    let crate_path = match args.value.get("crate_path") {
        Some(Expr::Lit(ExprLit {
            lit: Lit::Str(c), ..
        })) => match c.parse_with(Path::parse_mod_style) {
            Ok(c) => c,
            Err(e) => return e.to_compile_error().into(),
        },
        Some(c) => return error!(c.span() => "must be a string containing a path, eg. `\"as\"`"),
        None => Ident::new("adventure_script", input.span()).into(),
    };
    let deprecated = args.named.iter().find(|c| *c == "deprecated").is_some();

    //TODO:
    //  1. analyze signature
    //     - check if any have the #[wrap_from(Range)] attribute
    //     - check the return type is anyhow::Result
    //  2. add trait bounds:
    //     - IsAsVar for every variable
    //     - IntoIter for wrap_from
    //  3. construct the wrapper function
    //  4. construct the command

    let mut signature = vec![];
    for (item, _) in func.params.iter() {
        let FnParam::Typed(item) = item else {
            return error!("commands cannot take `self` as a parameter")
        };

        let wrap_to_result = item
            .attributes
            .iter()
            .find(|c| c.get_single_path_segment().is_some_and(|c| *c == "wrap_to"))
            .map(|c| syn::parse2::<Expr>(c.value.to_token_stream()));

        let wrap_to = match wrap_to_result {
            None => None,
            Some(Ok(Expr::Paren(ExprParen {expr, ..}))) => if let Expr::Range(c) = *expr {
                Some(c)
            } else {return error!(expr.span() => "expected a range")},
            Some(Ok(_)) => unreachable!(),
            Some(Err(e)) => return e.into_compile_error().into(),
        };

        signature.push(SignatureItem {
            name: item.name.clone(),
            ty: item.ty.clone(),
            wrap_to,
        })
    }

    let mut signature = signature.into_iter();

    let info = match signature.next() {
        None => {
            return error!(
                "function must have at least one argument, which has to be type `&mut GameInfo`"
            )
        }
        Some(c) => c,
    };

    let mut wrapping_code = quote! {};
    for (c, item) in signature.enumerate() {
        let name = item.name;
        let ty = item.ty;
        if let Some(c) = item.wrap_to {
            match (&c.start, &c.end) {
                (Some(start), Some(end)) => return todo!(),
                _ => return error!(c.span() => "range must have an explicit start and end"),
            }
        }
        quote! {
            let name = #ty.from_adventure_var(args[#c]);
        };
    }

    quote! {
        //TODO: remove this when done
        #[allow(unreachable_code)]
        //TODO: figure out the result type
        pub fn #fn_name () -> #crate_path::Result<#crate_path::core::Command> {
            Ok(#crate_path::core::Command {
                name: String::from(#name),
                func: todo!(),
                args: todo!(),
                deprecated: #deprecated,
            })
        }
    }
    .into()
}
