use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Colon, Plus};

use crate::parse::{DelegateComponentEntries, SimpleType, TypeSpec};

pub struct DefinePreset {
    pub preset: TypeSpec,
    pub parent_presets: Punctuated<PresetParent, Plus>,
    pub delegate_entries: DelegateComponentEntries<SimpleType>,
}

impl Parse for DefinePreset {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let PresetHead {
            preset,
            parent_presets,
        } = input.parse()?;

        let delegate_entries = input.parse()?;

        Ok(Self {
            preset,
            parent_presets,
            delegate_entries,
        })
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
