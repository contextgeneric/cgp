use cgp_macro_core::define_keyword;
use cgp_macro_core::types::keyword::Keyword;
use proc_macro2::TokenStream;
use syn::braced;
use syn::parse::{Parse, ParseStream};
use syn::token::Not;

use crate::types::MacroSnapshot;

define_keyword!(DelegateComponents, "delegate_components");

pub struct AssertDelegateComponents {
    pub body: TokenStream,
    pub snapshot: MacroSnapshot,
}

impl Parse for AssertDelegateComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Keyword<DelegateComponents> = input.parse()?;
        let _: Not = input.parse()?;

        let body = {
            let body;
            braced!(body in input);
            body.parse()?
        };

        let snapshot = input.parse()?;
        Ok(Self { body, snapshot })
    }
}
