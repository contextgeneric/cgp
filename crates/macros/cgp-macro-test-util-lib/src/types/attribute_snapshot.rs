use core::marker::PhantomData;

use cgp_macro_core::traits::IsKeyword;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::token::Pound;

use crate::functions::parse_attribute_with_keyword;
use crate::types::MacroSnapshot;

pub struct AttributeMacroSnapshot<Keyword, Item> {
    pub attr: TokenStream,
    pub body: Item,
    pub snapshot: MacroSnapshot,
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
