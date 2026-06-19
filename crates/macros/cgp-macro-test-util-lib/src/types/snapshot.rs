use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Ident, LitStr, braced, parenthesized};

use crate::functions::pretty_format;

pub struct MacroSnapshot {
    pub test_name: Ident,
    pub arg_ident: Ident,
    pub expr: TokenStream,
}

impl MacroSnapshot {
    pub fn wrap_output(&self, output: TokenStream) -> syn::Result<TokenStream> {
        let Self {
            test_name,
            arg_ident,
            expr,
        } = self;

        let output_string = pretty_format(output.clone())?;
        let output_literal = LitStr::new(&output_string, output.span());

        let out = quote! {
            #output

            #[test]
            fn #test_name() {
                let #arg_ident = #output_literal;

                #expr
            }
        };

        Ok(out)
    }
}

impl Parse for MacroSnapshot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
        })
    }
}
