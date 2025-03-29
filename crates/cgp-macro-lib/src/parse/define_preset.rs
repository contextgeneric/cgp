use syn::parse::{Parse, ParseStream};

use crate::parse::{DelegateComponentEntries, TypeSpec};

pub struct DefinePreset {
    pub preset: TypeSpec,
    pub delegate_entries: DelegateComponentEntries,
}

impl Parse for DefinePreset {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset = input.parse()?;

        let delegate_entries: DelegateComponentEntries = input.parse()?;

        Ok(Self {
            preset,
            delegate_entries,
        })
    }
}
