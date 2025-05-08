use core::iter;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, For, Lt, Semi};
use syn::{braced, bracketed, Ident, Type};

use crate::parse::ImplGenerics;

pub struct DelegateAndCheckSpec {
    pub impl_generics: ImplGenerics,
    pub trait_name: Ident,
    pub context_type: Type,
    pub provider_type: Type,
    pub entries: Punctuated<DelegateAndCheckEntry, Comma>,
}

#[derive(Clone)]
pub struct DelegateAndCheckEntry {
    pub keys: Punctuated<Type, Comma>,
    pub value: Type,
}

impl Parse for DelegateAndCheckSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let trait_name = input.parse()?;

        let _: For = input.parse()?;

        let context_type = input.parse()?;

        let _: Semi = input.parse()?;

        let provider_type = input.parse()?;

        let entries = {
            let body;
            braced!(body in input);
            Punctuated::parse_terminated(&body)?
        };

        Ok(Self {
            impl_generics,
            trait_name,
            context_type,
            provider_type,
            entries,
        })
    }
}

impl Parse for DelegateAndCheckEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keys = if input.peek(Bracket) {
            let body;
            bracketed!(body in input);
            Punctuated::parse_terminated(&body)?
        } else {
            let key: Type = input.parse()?;
            Punctuated::from_iter(iter::once(key))
        };

        let _: Colon = input.parse()?;

        let value = input.parse()?;

        Ok(Self { keys, value })
    }
}
