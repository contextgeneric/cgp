use core::marker::PhantomData;

use cgp_macro_core::traits::IsKeyword;
use cgp_macro_core::types::keyword::Keyword;
use proc_macro2::TokenStream;
use syn::braced;
use syn::parse::{Parse, ParseStream};
use syn::token::Not;

use crate::types::MacroSnapshot;

pub struct StatementMacroSnapshot<Keyword> {
    pub body: TokenStream,
    pub snapshot: MacroSnapshot,
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
