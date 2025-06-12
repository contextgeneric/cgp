#![no_std]

/*!
   This library provides helper macros for using async functions in traits.
*/

extern crate alloc;
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

/**
    The `#[async_trait]` macro is used to desugar async functions in traits
    to return `impl Future + Send`.

    This macro is required mainly to get around the current limitation of
    [async functions in traits](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/),
    which would produce a lint warning for `async_fn_in_trait` if bare async
    functions are defined in a trait.

    The default approach adopted by CGP is to always include a `Send` bound
    on the desugared `Future`, so that any async function that indirectly calls
    the method can still implement `Send` be be used with libraries such as `tokio::spawn`.

    It is possible to remove default `Send` by deactivating the `send` feature in the `cgp-async`
    crate. However, the removal may be unreliable, as the feature flag could still be accidentally
    enabled by one of the dependencies.

    Note that although the macro shares the same name as the
    [`async-trait`](https://docs.rs/async-trait) crate, it is implemented very differently.
    The other crate with the same name returns a `Pin<Box<dyn Future + Send>>`, while
    this macro returns a `impl Future + Send`.

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
        fn run(&self) -> impl Future<Output = ()> + Send;
    }
    ```
*/
#[proc_macro_attribute]
pub fn native_async(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    impl_async::impl_async(stream.into()).into()
}
