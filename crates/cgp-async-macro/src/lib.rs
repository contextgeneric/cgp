extern crate proc_macro;

use proc_macro::TokenStream;

mod helper;

#[proc_macro_attribute]
pub fn strip_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    helper::strip_async(stream.into()).into()
}

#[proc_macro_attribute]
pub fn native_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    stream
}
