use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma};
use syn::{Ident, Type, bracketed};

pub struct Entry {
    pub key: Ident,
    pub value: TokenStream,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let _colon: Colon = input.parse()?;
        let value = if input.peek(Bracket) {
            let body;
            bracketed!(body in input);
            let inner: TokenStream = body.parse()?;
            quote! { [ #inner ] }
        } else {
            input.parse::<Type>()?.to_token_stream()
        };

        Ok(Entry { key, value })
    }
}

pub struct Entries {
    pub entries: BTreeMap<String, TokenStream>,
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
