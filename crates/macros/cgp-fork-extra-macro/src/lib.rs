#![no_std]

/*!
   This crate provides the proc macros that define handler providers from
   functions, `#[cgp_computer]` and `#[cgp_producer]`, the
   `#[cgp_auto_dispatch]` macro that extends a per-type trait to extensible
   enums, and `#[cgp_auto_log]`. Each one forwards to `cgp-fork-extra-macro-lib`.
*/

use proc_macro::TokenStream;

/**
    `#[cgp_producer]` defines a `Producer` provider from a function that takes
    no input.

    The provider is named after the function in PascalCase, or after the
    identifier given as the attribute's argument. It is wired so the same
    function answers every member of the handler family, each returning the
    produced value whatever input it is given. The function must be
    synchronous, with no parameters, no generic parameters, and no
    `impl Trait` return type.

    ## Example

    ```rust,ignore
    #[cgp_producer]
    fn magic_number() -> u64 {
        42
    }
    ```

    defines the provider `MagicNumber`, so `MagicNumber::produce(&context, code)`
    returns `42`.
*/
#[proc_macro_attribute]
pub fn cgp_producer(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_fork_extra_macro_lib::cgp_producer(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/**
    `#[cgp_computer]` defines a `Computer` provider from a function.

    The function's parameters become the provider's input and its return type
    its output. A `#[implicit]` or `#[field]` parameter is read from a
    same-named field of the context instead of the input tuple. A synchronous
    function implements `Computer` and an `async` one `AsyncComputer`, and a
    return type written `Result<T, E>` makes the fallible members of the
    handler family propagate the error. The provider is wired so the same
    function answers every member of the family it can reach. The provider is
    named after the function in PascalCase, or after the identifier given as
    the attribute's argument.

    ## Example

    ```rust,ignore
    #[cgp_computer]
    fn add(a: u64, b: u64) -> u64 {
        a + b
    }
    ```

    defines the provider `Add`, so `Add::compute(&context, code, (1, 2))`
    returns `3`.
*/
#[proc_macro_attribute]
pub fn cgp_computer(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_fork_extra_macro_lib::cgp_computer(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/**
    `#[cgp_auto_dispatch]` implements a trait for every extensible enum whose
    variants' payloads implement it.

    The trait is kept unchanged. The macro adds a blanket impl that matches the
    enum's current variant and calls the same method on its payload, plus one
    per-variant `Computer` provider per method, named `Compute` followed by the
    method name in PascalCase. Every method must take `self`, `&self`, or
    `&mut self`, and must not have type or const generic parameters; the
    attribute takes no arguments.

    ## Example

    ```rust,ignore
    #[cgp_auto_dispatch]
    pub trait HasArea {
        fn area(&self) -> f64;
    }

    #[derive(CgpData)]
    pub enum Shape {
        Circle(Circle),
        Rectangle(Rectangle),
    }
    ```

    With `HasArea` implemented for `Circle` and `Rectangle`, `Shape` implements
    it too.
*/
#[proc_macro_attribute]
pub fn cgp_auto_dispatch(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_fork_extra_macro_lib::cgp_auto_dispatch(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Blanket-implements a logger trait by packing each method's arguments into a
/// detail struct and calling `CanLog`.
///
/// The trait's methods take `&self` and have no body. Each method becomes a
/// call to `self.log` with a struct named after the method (`log_hello` →
/// `__LogHello`). A context implements the trait when it implements `CanLog`
/// for those detail structs.
///
/// ```rust,ignore
/// #[cgp_auto_log]
/// pub trait CanLogHello {
///     fn log_hello(&self, name: &str);
/// }
/// ```
#[proc_macro_attribute]
pub fn cgp_auto_log(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_fork_extra_macro_lib::cgp_auto_log(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
