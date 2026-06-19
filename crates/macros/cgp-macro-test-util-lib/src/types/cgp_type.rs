use proc_macro2::TokenStream;
use syn::ItemTrait;
use syn::parse::{Parse, ParseStream};
use syn::token::Pound;

use crate::functions::parse_attribute;
use crate::types::MacroSnapshot;

pub struct SnapshotCgpType {
    pub attr: TokenStream,
    pub body: ItemTrait,
    pub snapshot: MacroSnapshot,
}

impl Parse for SnapshotCgpType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Pound = input.parse()?;

        let attr = parse_attribute("cgp_type", input)?;

        let body = input.parse()?;

        let snapshot = input.parse()?;

        Ok(Self {
            attr,
            body,
            snapshot,
        })
    }
}
