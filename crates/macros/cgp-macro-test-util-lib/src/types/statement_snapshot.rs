use core::marker::PhantomData;

use cgp_macro_core::traits::IsKeyword;
use cgp_macro_core::types::keyword::Keyword;
use proc_macro2::TokenStream;
use syn::braced;
use syn::parse::{Parse, ParseStream};
use syn::token::Not;

use crate::types::MacroSnapshot;

/// The input of a statement-macro snapshot: the body and the assertion.
pub struct StatementMacroSnapshot<Keyword> {
    /// The macro's body tokens.
    pub body: TokenStream,
    /// The snapshot's test name and assertion.
    pub snapshot: MacroSnapshot,
    /// The keyword marker.
    pub phantom: PhantomData<Keyword>,
}

impl<K: IsKeyword> Parse for StatementMacroSnapshot<K> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Keyword<K> = input.parse()?;
        let _: Not = input.parse()?;

        let body = {
            let body;
            braced!(body in input);
            body.parse()?
        };

        let snapshot = input.parse()?;
        Ok(Self {
            body,
            snapshot,
            phantom: PhantomData,
        })
    }
}
