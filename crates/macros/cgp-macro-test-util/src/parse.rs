use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::token::{At, Colon, Semi};
use syn::{Ident, LitStr};

pub struct MacroSnapshot {
    pub test_name: Ident,
    pub body: TokenStream,
    pub snapshot: LitStr,
}

impl Parse for MacroSnapshot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let test_name = input.parse()?;

        let _: Colon = input.parse()?;

        let _: At = input.parse()?;
        let snapshot = input.parse()?;
        let _: Semi = input.parse()?;

        let body = input.parse()?;

        Ok(MacroSnapshot {
            test_name,
            body,
            snapshot,
        })
    }
}
