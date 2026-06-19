use proc_macro2::TokenStream;
use syn::ItemFn;
use syn::parse::{Parse, ParseStream};
use syn::token::Pound;

use crate::functions::parse_attribute;
use crate::types::MacroSnapshot;

pub struct SnapshotCgpFn {
    pub attr: TokenStream,
    pub body: ItemFn,
    pub snapshot: MacroSnapshot,
}

impl Parse for SnapshotCgpFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Pound = input.parse()?;

        let attr = parse_attribute("cgp_fn", input)?;

        let body = input.parse()?;

        let snapshot = input.parse()?;

        Ok(Self {
            attr,
            body,
            snapshot,
        })
    }
}
