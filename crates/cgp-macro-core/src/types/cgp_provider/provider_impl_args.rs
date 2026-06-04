use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Error, GenericArgument, Lifetime, Type};

use crate::types::generics::GenericArguments;

pub struct ProviderImplArgs {
    pub context_type: Type,
    pub impl_args: Punctuated<ProviderImplArg, Comma>,
}

pub enum ProviderImplArg {
    Type(Type),
    Life(Lifetime),
}

impl ProviderImplArgs {
    pub fn from_generic_args(generic_args: &GenericArguments) -> syn::Result<Self> {
        let mut impl_args: Punctuated<ProviderImplArg, Comma> = Punctuated::new();
        let mut context_type: Option<Type> = None;

        if let Some(args) = &generic_args.args {
            for arg in &args.args {
                match arg {
                    GenericArgument::Lifetime(life) => {
                        impl_args.push(ProviderImplArg::Life(life.clone()));
                    }
                    GenericArgument::Type(ty) => {
                        if context_type.is_none() {
                            context_type = Some(ty.clone());
                        } else {
                            impl_args.push(ProviderImplArg::Type(ty.clone()));
                        }
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
