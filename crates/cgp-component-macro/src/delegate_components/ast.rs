use core::iter;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Gt, Lt};
use syn::{braced, bracketed, GenericParam, Generics, Ident, Token, Type};

pub struct DelegateComponentsAst {
    pub target_type: Type,
    pub target_generics: Generics,
    pub delegate_entries: Punctuated<DelegateEntryAst, Comma>,
}

pub struct DelegateEntryAst {
    pub components: Punctuated<ComponentAst, Comma>,
    pub source: Type,
}

pub struct ComponentAst {
    pub component_type: Type,
    pub component_generics: Generics,
}

impl Parse for DelegateComponentsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            input.parse()?
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
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(ComponentAst::parse, Token![,])?
        } else {
            let component: ComponentAst = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let _: Colon = input.parse()?;

        let source: Type = input.parse()?;

        Ok(Self { components, source })
    }
}

impl Parse for ComponentAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let component_type: Type = input.parse()?;

        Ok(Self {
            component_type,
            component_generics,
        })
    }
}
