use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::braced;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Colon, Comma, Override, Plus};

use crate::parse::{DelegateEntry, SimpleType, TypeSpec};

pub struct DefinePreset {
    pub preset: TypeSpec,
    pub parent_presets: Punctuated<PresetParent, Plus>,
    pub delegate_entries: Punctuated<DelegatePresetEntry, Comma>,
}

pub struct DelegatePresetEntry {
    pub is_override: Option<Override>,
    pub entry: DelegateEntry<SimpleType>,
}

impl Parse for DefinePreset {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let PresetHead {
            preset,
            parent_presets,
        } = input.parse()?;

        let delegate_entries = {
            let inner;
            braced!(inner in input);
            Punctuated::parse_terminated(&inner)?
        };

        Ok(Self {
            preset,
            parent_presets,
            delegate_entries,
        })
    }
}

impl Parse for DelegatePresetEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_override = if input.peek(Override) {
            let is_override = input.parse()?;
            Some(is_override)
        } else {
            None
        };

        let entry = input.parse()?;

        Ok(Self { is_override, entry })
    }
}

#[derive(Clone)]
pub struct PresetParent {
    pub has_expanded: Option<At>,
    pub parent_type: SimpleType,
}

impl Parse for PresetParent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let has_expanded = input.parse()?;
        let parent_type = input.parse()?;

        Ok(Self {
            has_expanded,
            parent_type,
        })
    }
}

impl ToTokens for DelegatePresetEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.is_override.to_tokens(tokens);
        self.entry.to_tokens(tokens);
    }
}

impl ToTokens for PresetParent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.has_expanded.to_tokens(tokens);
        self.parent_type.to_tokens(tokens);
    }
}

pub struct PresetHead {
    pub preset: TypeSpec,
    pub parent_presets: Punctuated<PresetParent, Plus>,
}

impl Parse for PresetHead {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset = input.parse()?;

        let parent_presets = if input.peek(Colon) {
            let _: Colon = input.parse()?;
            Punctuated::parse_separated_nonempty(input)?
        } else {
            Default::default()
        };

        Ok(Self {
            preset,
            parent_presets,
        })
    }
}
