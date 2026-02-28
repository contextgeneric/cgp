use core::iter;

use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Bracket, Comma, Lt, Pound};
use syn::{Attribute, Ident, Type, braced, bracketed, parse2};

use crate::parse::{DelegateMode, ImplGenerics, SimpleType};

pub struct DelegateAndCheckSpec {
    pub impl_generics: ImplGenerics,
    pub trait_name: Ident,
    pub context_type: Type,
    pub entries: Punctuated<DelegateAndCheckEntry, Comma>,
}

#[derive(Clone)]
pub struct DelegateAndCheckEntry {
    pub keys: Punctuated<Type, Comma>,
    pub mode: DelegateMode,
    pub value: Type,
}

impl Parse for DelegateAndCheckSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let m_trait_name = if input.peek(Pound) {
            let _: Pound = input.parse()?;
            let attributes = input.call(Attribute::parse_outer)?;

            let [attribute]: [Attribute; 1] = attributes.try_into().map_err(|_| {
                input.error("Expected exactly one attribute for the check trait name")
            })?;

            let ident: Ident = attribute.parse_args()?;
            Some(ident)
        } else {
            None
        };

        let context_type: Type = input.parse()?;

        let trait_name = match m_trait_name {
            Some(ident) => ident,
            None => {
                let context_type: SimpleType = parse2(context_type.to_token_stream())?;
                Ident::new(&format!("CanUse{}", context_type.name), context_type.span())
            }
        };

        let entries = {
            let body;
            braced!(body in input);
            Punctuated::parse_terminated(&body)?
        };

        Ok(Self {
            impl_generics,
            trait_name,
            context_type,
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

        let mode = input.parse()?;

        let value = input.parse()?;

        Ok(Self { keys, mode, value })
    }
}
