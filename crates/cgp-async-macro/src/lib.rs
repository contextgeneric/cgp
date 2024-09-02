/*!
   This library provides helper macros for using async functions in traits.
*/

extern crate proc_macro;

use proc_macro::TokenStream;

mod impl_async;
mod strip_async;

/**
   This macro can be used in place of the [`macro@native_async`] macro
   to strip away all use of `async` and `.await` syntax. This helps emulate
   async-generic by turnining async functions into sync functions.
*/
#[proc_macro_attribute]
pub fn strip_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    strip_async::strip_async(stream.into()).into()
}

#[proc_macro_attribute]
pub fn native_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    impl_async::impl_async(stream.into()).into()
}
