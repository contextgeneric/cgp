use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};
use syn::{Error, Ident, Type, braced, parse_quote};

use crate::types::delegate_component::{DelegateEntry, EvalDelegateValue};
use crate::types::generics::TypeGenerics;

pub struct DelegateValueWithInnerEntries {
    pub wrapper_ident: Ident,
    pub struct_ident: Ident,
    pub struct_generics: TypeGenerics,
    pub entries: Punctuated<DelegateEntry, Comma>,
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
            struct_generics,
            entries,
        })
    }
}

impl EvalDelegateValue for DelegateValueWithInnerEntries {
    fn eval(&self) -> syn::Result<Type> {
        let wrapper_ident = &self.wrapper_ident;
        let struct_ident = &self.struct_ident;
        let struct_generics = &self.struct_generics;

        let ty = parse_quote!( #wrapper_ident < #struct_ident #struct_generics > );
        Ok(ty)
    }
}
