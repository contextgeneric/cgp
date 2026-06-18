use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, braced, parenthesized};

pub struct MacroSnapshot {
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
