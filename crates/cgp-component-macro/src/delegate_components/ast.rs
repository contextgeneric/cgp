use core::iter;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Gt, Lt};
use syn::{braced, bracketed, GenericParam, Generics, Ident, Token, Type};

pub struct DelegateComponentsAst {
    pub target_type: Type,
    pub target_generics: Punctuated<GenericParam, Comma>,
    pub delegate_entries: Punctuated<DelegateEntryAst, Comma>,
}

pub struct DelegateEntryAst {
    pub components: Punctuated<Type, Comma>,
    pub source: Type,
}

impl Parse for DelegateComponentsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            let _: Lt = input.parse()?;

            let target_generics: Punctuated<GenericParam, Comma> =
                Punctuated::parse_separated_nonempty(input)?;

            let _: Gt = input.parse()?;

            target_generics
        } else {
            Default::default()
        };

        let target_type: Type = input.parse()?;

        let delegate_entries = {
            let entries_body;
            braced!(entries_body in input);
            entries_body.parse_terminated(DelegateEntryAst::parse, Token![,])?
        };

        Ok(Self {
            target_type,
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
