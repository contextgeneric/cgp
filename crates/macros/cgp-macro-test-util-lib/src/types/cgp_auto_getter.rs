use proc_macro2::TokenStream;
use syn::ItemTrait;
use syn::parse::{Parse, ParseStream};
use syn::token::Pound;

use crate::functions::parse_attribute;
use crate::types::MacroSnapshot;

pub struct SnapshotCgpAutoGetter {
    pub attr: TokenStream,
    pub body: ItemTrait,
    pub snapshot: MacroSnapshot,
}

impl Parse for SnapshotCgpAutoGetter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Pound = input.parse()?;

        let attr = parse_attribute("cgp_auto_getter", input)?;

        let body = input.parse()?;

        let snapshot = input.parse()?;

        Ok(Self {
            attr,
            body,
            snapshot,
        })
    }
}
