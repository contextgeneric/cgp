mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse2;

use crate::parse::MacroSnapshot;

#[proc_macro]
pub fn assert_delegate_components(body: TokenStream) -> TokenStream {
    let MacroSnapshot {
        test_name,
        body,
        snapshot,
    } = parse2(body.into()).unwrap();

    let output =
        cgp_macro_lib::delegate_components(body).unwrap_or_else(syn::Error::into_compile_error);

    let wrapped = quote! {
        #output

        #[test]
        fn #test_name() {
            insta::assert_snapshot!(
                prettyplease::unparse(&syn::parse2(quote::quote! {
                    #output
                }).unwrap()),
                @#snapshot,
            );
        }
    };

    wrapped.into()
}
