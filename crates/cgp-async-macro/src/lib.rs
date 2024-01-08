extern crate proc_macro;

use proc_macro::TokenStream;

mod impl_async;
mod strip_async;

#[proc_macro_attribute]
pub fn strip_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    strip_async::strip_async(stream.into()).into()
}

#[proc_macro_attribute]
pub fn native_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    impl_async::impl_async(stream.into()).into()
}
