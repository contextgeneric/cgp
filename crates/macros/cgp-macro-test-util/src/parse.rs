use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Pound;
use syn::{Ident, braced, bracketed, parenthesized};

pub struct MacroSnapshot {
    pub attrs: Option<TokenStream>,
    pub body: TokenStream,
    pub test_name: Ident,
    pub arg_ident: Ident,
    pub expr: TokenStream,
}

impl MacroSnapshot {
    pub fn wrap_output(&self, output: TokenStream) -> TokenStream {
        let Self {
            test_name,
            arg_ident,
            expr,
            ..
        } = self;

        quote! {
            #output

            #[test]
            fn #test_name() {
                let #arg_ident = cgp_macro_core::functions::pretty_format(quote::quote! {
                    #output
                }).unwrap();

                #expr
            }
        }
    }
}

impl Parse for MacroSnapshot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = if input.peek(Pound) {
            let _: Pound = input.parse()?;

            let attrs_body;
            bracketed!(attrs_body in input);

            Some(attrs_body.parse()?)
        } else {
            None
        };

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
            attrs,
            body,
            test_name,
            arg_ident,
            expr,
        })
    }
}
