use core::iter;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma};
use syn::{braced, bracketed, Generics, Ident, Token, Type};

pub struct DelegateComponentsAst {
    pub target_ident: Ident,
    pub target_generics: Generics,
    pub delegate_entries: Punctuated<DelegateEntryAst, Comma>,
}

pub struct DelegateEntryAst {
    pub components: Punctuated<Type, Comma>,
    pub source: Type,
}

impl Parse for DelegateComponentsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_ident: Ident = input.parse()?;

        let target_generics: Generics = input.parse()?;

        let delegate_entries = {
            let entries_body;
            braced!(entries_body in input);
            entries_body.parse_terminated(DelegateEntryAst::parse, Token![,])?
        };

        Ok(Self {
            target_ident,
            target_generics,
            delegate_entries,
        })
    }
}

impl Parse for DelegateEntryAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components: Punctuated<Type, Comma> = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(Type::parse, Token![,])?
        } else {
            let component: Type = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let _: Colon = input.parse()?;

        let source: Type = input.parse()?;

        Ok(Self { components, source })
    }
}
