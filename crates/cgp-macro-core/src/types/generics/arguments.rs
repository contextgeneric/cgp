use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Lt};
use syn::{AngleBracketedGenericArguments, Error, GenericArgument, Type, parse_quote};

use crate::types::generics::TypeGenerics;

#[derive(Debug, Clone, Default)]
pub struct GenericArguments {
    pub args: Option<AngleBracketedGenericArguments>,
}

impl GenericArguments {
    pub fn make_args(&mut self) -> &mut Punctuated<GenericArgument, Comma> {
        &mut self.args.get_or_insert_with(|| parse_quote!(<>)).args
    }

    pub fn type_args(&self) -> Vec<Type> {
        let mut params: Vec<Type> = Vec::new();

        if let Some(args) = &self.args {
            for arg in &args.args {
                match arg {
                    GenericArgument::Type(ty) => {
                        params.push(ty.clone());
                    }
                    _ => {}
                }
            }
        }

        params
    }

    /// Convert the arguments to a list of types to be used in `IsProviderFor`.
    /// This mainly converts the lifetimes `'a` into `Life<'a>` so that they can
    /// be used as types.
    ///
    /// Other generic parameters like const generics arguments are currently
    /// unsupported.
    pub fn to_param_types(&self) -> syn::Result<Vec<Type>> {
        let mut params: Vec<Type> = Vec::new();

        if let Some(args) = &self.args {
            for arg in &args.args {
                match arg {
                    GenericArgument::Lifetime(life) => {
                        params.push(parse_quote! { Life<#life> });
                    }
                    GenericArgument::Type(ty) => {
                        params.push(ty.clone());
                    }
                    _ => {
                        return Err(Error::new(
                            arg.span(),
                            format!("unsupported type argument: {:?}", arg),
                        ));
                    }
                }
            }
        }

        Ok(params)
    }
}

impl Parse for GenericArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lt) {
            let args = input.parse()?;
            Ok(Self { args: Some(args) })
        } else {
            Ok(Self { args: None })
        }
    }
}

impl ToTokens for GenericArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(args) = &self.args {
            args.to_tokens(tokens);
        }
    }
}

impl From<TypeGenerics> for GenericArguments {
    fn from(generics: TypeGenerics) -> Self {
        if generics.params.is_empty() {
            Self { args: None }
        } else {
            let args = parse_quote!(#generics);
            Self { args: Some(args) }
        }
    }
}
