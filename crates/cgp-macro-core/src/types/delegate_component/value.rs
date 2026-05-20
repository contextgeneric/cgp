use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};
use syn::{Error, Generics, Ident, Type, braced};

use crate::types::delegate_component::DelegateEntry;
use crate::types::generics::TypeGenerics;

pub enum DelegateValue {
    Type(Type),
    WithInner(DelegateValueWithInnerEntries),
}

pub struct DelegateValueWithInnerEntries {
    pub wrapper_ident: Ident,
    pub struct_ident: Ident,
    pub struct_generics: Generics,
    pub entries: Punctuated<DelegateEntry, Comma>,
}

impl Parse for DelegateValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if let Ok(value) = fork.parse::<DelegateValueWithInnerEntries>() {
            input.advance_to(&fork);
            return Ok(Self::WithInner(value));
        }

        let ty: Type = input.parse()?;
        Ok(Self::Type(ty))
    }
}

impl Parse for DelegateValueWithInnerEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper_ident = input.parse()?;

        let _: Lt = input.parse()?;

        let new_ident: Ident = input.parse()?;

        if new_ident != "new" {
            return Err(Error::new(new_ident.span(), "expect `new` keyword"));
        }

        let struct_ident = input.parse()?;

        let struct_generics: TypeGenerics = input.parse()?;

        let entries = {
            let content;
            braced!(content in input);

            Punctuated::parse_terminated(&content)?
        };

        let _: Gt = input.parse()?;

        Ok(Self {
            wrapper_ident,
            struct_ident,
            struct_generics: struct_generics.generics,
            entries,
        })
    }
}
