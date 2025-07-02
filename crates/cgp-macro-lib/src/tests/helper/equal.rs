use proc_macro2::TokenStream;

use crate::tests::helper::format::format_token_stream;

pub fn equal_token_stream(left: &TokenStream, right: &TokenStream) -> bool {
    format_token_stream(left) == format_token_stream(right)
}

pub fn assert_equal_token_stream(derived: &TokenStream, expected: &TokenStream) {
    let formatted_derived = format_token_stream(derived);
    let formatted_expected = format_token_stream(expected);

    if formatted_derived != formatted_expected {
        panic!(
            "token stream does not match. expected:\n{formatted_expected}\ngot:\n{formatted_derived}",
        );
    }
}
