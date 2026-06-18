use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, braced, parenthesized};

pub struct MacroSnapshot {
    pub test_name: Ident,
    pub arg_ident: Ident,
    pub expr: TokenStream,
    pub body: TokenStream,
}

impl Parse for MacroSnapshot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body = {
            let body;
            braced!(body in input);
            body.parse()?
        };

        let test_name = input.parse()?;

        let arg_ident = {
            let arg_body;
            parenthesized!(arg_body in input);
            arg_body.parse()?
        };

        let expr = {
            let expr_body;
            braced!(expr_body in input);
            expr_body.parse()?
        };

        Ok(MacroSnapshot {
            test_name,
            arg_ident,
            expr,
            body,
        })
    }
}
