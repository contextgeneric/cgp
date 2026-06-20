use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Error, Lifetime, Type};

use crate::types::ident::{TypeArg, TypeArgs};

pub struct ProviderImplArgs {
    pub context_type: Type,
    pub impl_args: Punctuated<ProviderImplArg, Comma>,
}

pub enum ProviderImplArg {
    Type(Type),
    Life(Lifetime),
}

impl ProviderImplArgs {
    pub fn from_generic_args(generic_args: &TypeArgs) -> syn::Result<Self> {
        let mut impl_args: Punctuated<ProviderImplArg, Comma> = Punctuated::new();
        let mut context_type: Option<Type> = None;

        for arg in &generic_args.args {
            match arg {
                TypeArg::Lifetime(life) => {
                    impl_args.push(ProviderImplArg::Life(life.clone()));
                }
                TypeArg::Type(ty) => {
                    if context_type.is_none() {
                        context_type = Some(ty.clone());
                    } else {
                        impl_args.push(ProviderImplArg::Type(ty.clone()));
                    }
                }
                TypeArg::Const(expr) => {
                    return Err(Error::new(
                        expr.span(),
                        "const arguments are not supported in provider impl trait arguments",
                    ));
                }
            }
        }

        let context_type = context_type.ok_or_else(|| Error::new(
            generic_args.span(),
            "provider impl should contain trait path containing at least one generic type parameter",
        ))?;

        Ok(Self {
            context_type,
            impl_args,
        })
    }
}

impl ToTokens for ProviderImplArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_args.to_tokens(tokens);
    }
}

impl ToTokens for ProviderImplArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ProviderImplArg::Type(ty) => {
                ty.to_tokens(tokens);
            }
            ProviderImplArg::Life(life) => {
                tokens.extend(quote!(Life<#life>));
            }
        }
    }
}
