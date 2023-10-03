use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{spanned::Spanned, Expr, ExprLit, ExprParen, ExprRange, Ident, Lit, Path, RangeLimits};
use venial::{FnParam, TyExpr};

use crate::util::AttrArgs;

struct SignatureItem {
    pub name: Ident,
    pub ty: TyExpr,
    pub wrap_to: Option<ExprRange>,
}

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
    let func_inner = match func.body {
        Some(c) => c.clone(),
        None => return error!("function must have a body"),
    };

    let args = match syn::parse::<AttrArgs>(args) {
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
    let deprecated = args.named.iter().any(|c| c == "deprecated");

    //TODO: check if the return type is anyhow::Result

    let mut signature = vec![];
    for (item, _) in func.params.iter() {
        let FnParam::Typed(item) = item else {
            return error!("commands cannot take `self` as a parameter");
        };

        let wrap_to_result = item
            .attributes
            .iter()
            .find(|c| {
                matches!(
                    c.get_single_path_segment().map(|c| *c == "wrap_to"),
                    Some(true)
                )
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

    let mut wrapping_code = quote! {};
    let mut args_value = quote! {};
    let mut arg_num = 0usize;
    for item in signature {
        let name = item.name;
        let name_str = name.to_string();
        let ty = item.ty;

        if let Some(c) = item.wrap_to {
            match (&c.start, &c.end) {
                (Some(start), Some(end)) => {
                    let is_closed = matches!(c.limits, RangeLimits::Closed(_));
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
                        let name_numbered: Ident = format_ident!("{}_{}", name, pos + 1);
                        names_numbered.push(name_numbered.clone());
                        let name_numbered_str = name_numbered.to_string();

                        wrapping_code = quote! {
                            #wrapping_code
                            let #name_numbered = {
                                type T = <#ty as #crate_path::core::ASVarWrapTo>::InnerType;
                                (
                                    #pos,
                                    T::from_adventure_var(&args[#arg_num + #pos])
                                )
                            };
                        };
                        let is_required = !bounds.contains(&pos);
                        args_value = quote! {
                            #args_value
                            #crate_path::core::commands::CommandArg {
                                name: String::from(#name_numbered_str),
                                type_:
                                    <#ty as #crate_path::core::ASVarWrapTo>::InnerType::ADVENTURE_TYPE,
                                required: #is_required,
                            },

                        }
                    }
                    arg_num += range_size;

                    wrapping_code = quote! {
                        #wrapping_code
                        let mut #name = vec![];
                        {
                            let mut is_done = false;
                            #(
                                if #names_numbered.0 < #start {
                                    // arg checking has already made sure this exists
                                    #name.push(#names_numbered.1.clone().unwrap());
                                } else if let Some(elmt) = &#names_numbered.1 {
                                    if is_done {
                                        todo!("error managing for '_3 exists _2 does not'")
                                    }
                                    #name.push(elmt.clone())
                                } else {
                                    is_done = true;
                                }
                                drop(#names_numbered);
                            )*
                        }
                        // again, types should be properly handled by the arg type checker
                        let #name = <#ty as #crate_path::core::ASVarWrapTo>::wrap(#name).unwrap();
                    }
                }
                _ => return error!(c.span() => "range must have an explicit start and end"),
            }
        } else {
            //TODO: error managing if !ty::IS_OPTIONAL and it's None
            wrapping_code = quote! {
                #wrapping_code
                let #name = #crate_path::core::specialization_hack::Wrap::<#ty>::new().refer().from_adventure_var(&args[#arg_num]);
                let #name = #crate_path::core::specialization_hack::Wrap::<#ty>::new().refer().unwrap_if_optional(#name);
            };
            args_value = quote! {
                #args_value
                #crate_path::core::commands::CommandArg {
                    name: String::from(#name_str),
                    type_: <#ty>::ADVENTURE_TYPE,
                    required: !#crate_path::core::specialization_hack::Wrap::<#ty>::new().refer().is_optional(),
                },
            };
            arg_num += 1;
        }
    }

    quote! {
        pub fn #fn_name () -> #crate_path::Result<#crate_path::core::Command> {
            //TODO: remove imports, use explicit <T as Trait>
            use #crate_path::core::{IsASVar, ASVarWrapTo};
            use #crate_path::core::specialization_hack::{OptionInfo};
            fn func (
                #info_name: &mut #crate_path::core::GameInfo,
                args: Vec<#crate_path::core::ASVariable>,
            ) -> #crate_path::Result<()> {
                #wrapping_code
                drop(args);
                #func_inner
            };
            let args = vec![#args_value];
            #crate_path::core::Command::new(String::from(#name), func, args, #deprecated)
        }
    }
    .into()
}
