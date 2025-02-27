use syn::parse::{Parse, ParseStream};

use crate::parse::{DelegateComponentEntries, TypeSpec};

pub struct DefinePresetAst {
    pub preset: TypeSpec,
    pub delegate_entries: DelegateComponentEntries,
}

impl Parse for DefinePresetAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let preset = input.parse()?;

        let delegate_entries: DelegateComponentEntries = input.parse()?;

        Ok(Self {
            preset,
            delegate_entries,
        })
    }
}
