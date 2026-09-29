use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cgp_producer(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_fork_extra_macro_lib::cgp_producer(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_computer(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_fork_extra_macro_lib::cgp_computer(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

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
