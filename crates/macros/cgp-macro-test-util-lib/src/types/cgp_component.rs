use proc_macro2::TokenStream;
use syn::ItemTrait;
use syn::parse::{Parse, ParseStream};
use syn::token::Pound;

use crate::functions::parse_attribute;
use crate::types::MacroSnapshot;

pub struct SnapshotCgpComponent {
    pub attr: TokenStream,
    pub body: ItemTrait,
    pub snapshot: MacroSnapshot,
}

impl Parse for SnapshotCgpComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Pound = input.parse()?;

        let attr = parse_attribute("cgp_component", input)?;

        let body = input.parse()?;

        let snapshot = input.parse()?;

        Ok(Self {
            attr,
            body,
            snapshot,
        })
    }
}
