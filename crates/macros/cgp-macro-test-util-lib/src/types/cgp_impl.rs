use cgp_macro_core::define_keyword;
use cgp_macro_core::types::keyword::Keyword;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::token::{Paren, Pound};
use syn::{ItemImpl, braced, bracketed, parenthesized};

use crate::types::MacroSnapshot;

define_keyword!(CgpImpl, "cgp_impl");

pub struct SnapshotCgpImpl {
    pub attr: TokenStream,
    pub body: ItemImpl,
    pub snapshot: MacroSnapshot,
}

impl Parse for SnapshotCgpImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Pound = input.parse()?;

        let attr = {
            let outer_body;
            bracketed!(outer_body in input);

            let _: Keyword<CgpImpl> = outer_body.parse()?;

            if outer_body.peek(Paren) {
                let inner_body;
                parenthesized!(inner_body in outer_body);
                inner_body.parse()?
            } else {
                let inner_body;
                braced!(inner_body in outer_body);
                inner_body.parse()?
            }
        };
        // let attr = TokenStream::new();

        let body = input.parse()?;

        let snapshot = input.parse()?;

        Ok(Self {
            attr,
            body,
            snapshot,
        })
    }
}
