use core::marker::PhantomData;

use cgp_macro_core::traits::IsKeyword;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::token::Pound;

use crate::functions::parse_attribute_with_keyword;
use crate::types::MacroSnapshot;

/// The input of an attribute-macro snapshot: the attribute, the item, and the assertion.
pub struct AttributeMacroSnapshot<Keyword, Item> {
    /// The attribute's inner tokens.
    pub attr: TokenStream,
    /// The annotated item.
    pub body: Item,
    /// The snapshot's test name and assertion.
    pub snapshot: MacroSnapshot,
    /// The keyword marker.
    pub phantom: PhantomData<Keyword>,
}

impl<K: IsKeyword, Item: Parse> Parse for AttributeMacroSnapshot<K, Item> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Pound = input.parse()?;

        let attr = parse_attribute_with_keyword::<K>(input)?;

        let body = input.parse()?;

        let snapshot = input.parse()?;

        Ok(Self {
            attr,
            body,
            snapshot,
            phantom: PhantomData,
        })
    }
}
