use proc_macro2::TokenStream;
use syn::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::Semi;

pub struct MacroSnapshot {
    pub test_name: Ident,
    pub body: TokenStream,
}

impl Parse for MacroSnapshot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let test_name = input.parse()?;

        let _: Semi = input.parse()?;

        let body = input.parse()?;

        Ok(MacroSnapshot { test_name, body })
    }
}
