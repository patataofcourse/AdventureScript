extern crate proc_macro;

use std::{collections::btree_map::Range, ops::RangeBounds};

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{
    spanned::Spanned, Expr, ExprLit, ExprParen, ExprRange, Ident, Lit, LitInt, Path, RangeLimits,
};
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
            .find(|c| {
                // ew
                if let Some(true) = c.get_single_path_segment().map(|c| *c == "wrap_to") {
                    true
                } else {
                    false
                }
            })
            .map(|c| syn::parse2::<Expr>(c.value.to_token_stream()));

        let wrap_to = match wrap_to_result {
            None => None,
            Some(Ok(Expr::Paren(ExprParen { expr, .. }))) => {
                if let Expr::Range(c) = *expr {
                    Some(c)
                } else {
                    return error!(expr.span() => "expected a range");
                }
            }
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
    let info_name = info.name;

    let mut wrapping_code = quote! {
        use #crate_path::core::variables::is_as_var::IsASVar;
    };
    let mut args_value = quote! {};
    let mut arg_num = 0usize;
    for item in signature {
        let name = item.name;
        let name_str = name.to_string();
        let ty = item.ty;
        if let Some(c) = item.wrap_to {
            match (&c.start, &c.end) {
                (Some(start), Some(end)) => {
                    let is_closed = if let RangeLimits::Closed(_) = c.limits {
                        true
                    } else {
                        false
                    };
                    let start = match start.as_ref() {
                        Expr::Lit(ExprLit {
                            lit: Lit::Int(c), ..
                        }) => {
                            match c
                                .base10_parse::<usize>()
                                .map_err(|e| syn::Error::new(c.span(), e).to_compile_error().into())
                            {
                                Ok(c) => c,
                                Err(e) => return e,
                            }
                        }
                        _ => {
                            return error!(start.span() => "wrap_to range bounds must be integer literals")
                        }
                    };
                    let end = match end.as_ref() {
                        Expr::Lit(ExprLit {
                            lit: Lit::Int(c), ..
                        }) => {
                            match c
                                .base10_parse::<usize>()
                                .map_err(|e| syn::Error::new(c.span(), e).to_compile_error().into())
                            {
                                Ok(c) => c,
                                Err(e) => return e,
                            }
                        }
                        _ => {
                            return error!(end.span() => "wrap_to range bounds must be integer literals")
                        }
                    };

                    let (range, range_size) = if is_closed {
                        (0..=(end - 1), (end - 1))
                    } else {
                        (0..=(end - 2), end - 2)
                    };
                    let bounds = range.clone().skip(start).collect::<Vec<_>>();

                    let mut names_numbered = vec![];
                    for pos in range {
                        let name_numbered: Ident = format_ident!("{}_{}", name, pos);
                        names_numbered.push(name_numbered.clone());
                        let name_numbered_str = name_numbered.to_string();

                        if bounds.contains(&pos) {
                        } else {
                            wrapping_code = quote! {
                                #wrapping_code
                                let #name_numbered = <#ty::INNER_TYPE>::from_adventure_var(&args[#arg_num + #pos]);
                            };
                            args_value = quote! {
                                #args_value
                                #crate_path::core::commands::CommandArg {
                                    name: String::from(#name_numbered_str),
                                    type_: #ty::ADVENTURE_TYPE,
                                    required: !#ty::IS_OPTIONAL,
                                },

                            }
                        }
                    }
                    arg_num += range_size;

                    wrapping_code = quote! {
                        #wrapping_code
                        let #name = T::wrap_from(vec![#(#names_numbered),*]);
                    }

                    //return todo!();
                }
                _ => return error!(c.span() => "range must have an explicit start and end"),
            }
        } else {
            wrapping_code = quote! {
                #wrapping_code
                let #name = #ty::from_adventure_var(&args[#arg_num]).ok_or()?;
            };
            args_value = quote! {
                #args_value
                #crate_path::core::commands::CommandArg {
                    name: String::from(#name_str),
                    type_: #ty::ADVENTURE_TYPE,
                    required: !#ty::IS_OPTIONAL,
                },
            };
            arg_num += 1;
        }
    }

    quote! {
        //TODO: remove this when done
        #[allow(unreachable_code)]
        //TODO: figure out the result type
        pub fn #fn_name () -> #crate_path::Result<#crate_path::core::Command> {
            use #crate_path::core::variables::is_as_var::{IsASVar, ASVarWrapTo};
            Ok(#crate_path::core::Command {
                name: String::from(#name),
                func: |#info_name, args| {
                    #wrapping_code
                    drop(args);
                    //TODO: function contents
                    Ok(())
                },
                args: vec![#args_value],
                deprecated: #deprecated,
            })
        }
    }
    .into()
}
