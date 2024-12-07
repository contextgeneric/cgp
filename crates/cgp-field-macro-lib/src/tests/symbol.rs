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
        type Symbol = Cons<
            Char<'h'>,
            Cons<
                Char<'e'>,
                Cons<
                    Char<'l'>,
                    Cons<
                        Char<'l'>,
                        Cons<
                            Char<'o'>,
                            Nil
                        >>>>>;
    };

    assert!(equal_token_stream(&derived, &expected));
}
