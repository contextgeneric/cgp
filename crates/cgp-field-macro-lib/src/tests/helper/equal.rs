use proc_macro2::TokenStream;

use crate::tests::helper::format::format_token_stream;

pub fn equal_token_stream(left: &TokenStream, right: &TokenStream) -> bool {
    format_token_stream(left) == format_token_stream(right)
}
