use core::iter;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Lt};
use syn::{braced, bracketed, Token, Type};

use crate::parse::ImplGenerics;

pub struct DelegateComponents {
    pub target_type: Type,
    pub target_generics: ImplGenerics,
    pub delegate_entries: DelegateComponentEntries,
}

pub struct DelegateComponentEntries {
    pub entries: Punctuated<DelegateComponentEntry, Comma>,
}

pub struct DelegateComponentEntry {
    pub components: Punctuated<DelegateComponentName, Comma>,
    pub source: Type,
}

#[derive(Clone)]
pub struct DelegateComponentName {
    pub component_type: Type,
    pub component_generics: ImplGenerics,
}

impl DelegateComponentEntries {
    pub fn all_components(&self) -> Punctuated<DelegateComponentName, Comma> {
        self.entries
            .iter()
            .flat_map(|entry| entry.components.clone().into_iter())
            .collect()
    }
}

impl Parse for DelegateComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let target_type: Type = input.parse()?;

        let delegate_entries: DelegateComponentEntries = input.parse()?;

        Ok(Self {
            target_type,
            target_generics,
            delegate_entries,
        })
    }
}

impl Parse for DelegateComponentEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entries = {
            let entries_body;
            braced!(entries_body in input);
            entries_body.parse_terminated(DelegateComponentEntry::parse, Comma)?
        };

        Ok(Self { entries })
    }
}

impl Parse for DelegateComponentEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(DelegateComponentName::parse, Token![,])?
        } else {
            let component: DelegateComponentName = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let _: Colon = input.parse()?;

        let source: Type = input.parse()?;

        Ok(Self { components, source })
    }
}

impl Parse for DelegateComponentName {
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

impl ToTokens for DelegateComponentName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.component_generics.to_token_stream());
        tokens.extend(self.component_type.to_token_stream());
    }
}
