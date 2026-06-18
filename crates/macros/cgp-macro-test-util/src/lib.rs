mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse2};

use crate::parse::MacroSnapshot;

#[proc_macro]
pub fn assert_delegate_components(body: TokenStream) -> TokenStream {
    let MacroSnapshot { test_name, body } = parse2(body.into()).unwrap();

    let output = cgp_macro_lib::delegate_components(body).unwrap();

    let test_name_lit = LitStr::new(&test_name.to_string(), test_name.span());

    let wrapped = quote! {
        #output

        #[test]
        fn #test_name() {
            let output = cgp_macro_core::functions::pretty_format(quote::quote! {
                #output
            }).unwrap();

            insta::assert_snapshot!(#test_name_lit, output);
        }
    };

    wrapped.into()
}
