use quote::quote;

use crate::symbol::make_symbol;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_symbol_macro() {
    let symbol = make_symbol(quote!("hello"));

    let derived = quote! {
        type Symbol = #symbol;
    };

    let expected = quote! {
        type Symbol = ι<'h', ι<'e', ι<'l', ι<'l', ι<'o', ε>>>>>;
    };

    assert!(equal_token_stream(&derived, &expected));
}
