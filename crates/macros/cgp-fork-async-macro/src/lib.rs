#![no_std]

/*!
   This library provides helper macros for using async functions in traits.
*/

extern crate alloc;
extern crate proc_macro;

use proc_macro::TokenStream;

mod impl_async;

/**
    The `#[async_trait]` macro is used to desugar async functions in traits
    to return `impl Future`.

    This macro is required mainly to get around the current limitation of
    [async functions in traits](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/),
    which would produce a lint warning for `async_fn_in_trait` if bare async
    functions are defined in a trait.

    ## Example

    Given the following trait definition:

    ```rust,ignore
    #[async_trait]
    pub trait CanRun {
        async fn run(&self);
    }
    ```

    The macro would desugar it to the following:

    ```rust,ignore
    pub trait CanRun {
        fn run(&self) -> impl Future<Output = ()>;
    }
    ```
*/
#[proc_macro_attribute]
pub fn async_trait(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    impl_async::impl_async(stream.into()).into()
}
