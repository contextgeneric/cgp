use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus};
use syn::Ident;

use crate::parse::{DelegateComponentEntries, TypeSpec};

pub struct DefinePreset {
    pub preset: TypeSpec,
    pub parent_presets: Punctuated<Ident, Plus>,
    pub delegate_entries: DelegateComponentEntries,
}

impl Parse for DefinePreset {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset = input.parse()?;

        let parent_presets = if input.peek(Colon) {
            let _: Colon = input.parse()?;
            // let parent: Ident = input.parse()?;
            // Punctuated::from_iter([parent])
            <Punctuated<Ident, Plus>>::parse_separated_nonempty(input)?
        } else {
            Default::default()
        };

        let delegate_entries: DelegateComponentEntries = input.parse()?;

        Ok(Self {
            preset,
            parent_presets,
            delegate_entries,
        })
    }
}
