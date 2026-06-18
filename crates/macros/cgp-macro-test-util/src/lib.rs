mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse2;

use crate::parse::MacroSnapshot;

#[proc_macro]
pub fn snapshot_delegate_components(body: TokenStream) -> TokenStream {
    let MacroSnapshot {
        test_name,
        arg_ident,
        expr,
        body,
    } = parse2(body.into()).unwrap();

    let output = cgp_macro_lib::delegate_components(body).unwrap();

    let wrapped = quote! {
        #output

        #[test]
        fn #test_name() {
            let #arg_ident = cgp_macro_core::functions::pretty_format(quote::quote! {
                #output
            }).unwrap();

            #expr
        }
    };

    wrapped.into()
}
