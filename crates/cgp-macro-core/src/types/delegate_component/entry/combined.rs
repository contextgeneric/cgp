use syn::parse::{Parse, ParseStream};

use crate::types::delegate_component::{DelegateMode, DirectDelegateEntry, NormalDelegateEntry};

pub enum DelegateEntry {
    Normal(NormalDelegateEntry),
    Direct(DirectDelegateEntry),
}

impl Parse for DelegateEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let mode: DelegateMode = input.parse()?;
        let value = input.parse()?;

        let entry = match mode {
            DelegateMode::Normal(colon) => Self::Normal(NormalDelegateEntry { key, colon, value }),
            DelegateMode::Direct(arrow) => Self::Direct(DirectDelegateEntry { key, arrow, value }),
        };

        Ok(entry)
    }
}
