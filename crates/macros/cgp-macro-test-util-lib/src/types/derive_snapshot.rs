use core::marker::PhantomData;

use cgp_macro_core::traits::IsKeyword;
use cgp_macro_core::types::keyword::Keyword;
use syn::parse::{Parse, ParseStream};
use syn::parse2;
use syn::token::Pound;

use crate::functions::parse_attribute_with_keyword;
use crate::keywords::Derive;
use crate::types::MacroSnapshot;

/// The input of a derive-macro snapshot: the item and the assertion.
pub struct DeriveMacroSnapshot<Keyword, Item> {
    /// The annotated item.
    pub body: Item,
    /// The snapshot's test name and assertion.
    pub snapshot: MacroSnapshot,
    /// The keyword marker.
    pub phantom: PhantomData<Keyword>,
}

impl<K: IsKeyword, Item: Parse> Parse for DeriveMacroSnapshot<K, Item> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Pound = input.parse()?;

        let attr = parse_attribute_with_keyword::<Derive>(input)?;

        let _keyword: Keyword<K> = parse2(attr)?;

        let body = input.parse()?;

        let snapshot = input.parse()?;

        Ok(Self {
            body,
            snapshot,
            phantom: PhantomData,
        })
    }
}
