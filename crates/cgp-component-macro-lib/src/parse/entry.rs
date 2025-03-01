use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma};
use syn::{Ident, Type};

pub struct Entry {
    pub key: Ident,
    pub value: Type,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let _colon: Colon = input.parse()?;
        let value = input.parse()?;

        Ok(Entry { key, value })
    }
}

pub struct Entries {
    pub entries: BTreeMap<String, Type>,
}

impl Parse for Entries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entry_list: Punctuated<Entry, Comma> = Punctuated::parse_terminated(input)?;

        let entries = BTreeMap::from_iter(
            entry_list
                .into_iter()
                .map(|entry| (entry.key.to_string(), entry.value)),
        );

        Ok(Entries { entries })
    }
}
