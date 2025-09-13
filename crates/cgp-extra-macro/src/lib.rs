use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cgp_producer(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_extra_macro_lib::cgp_producer(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_computer(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_extra_macro_lib::cgp_computer(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_auto_dispatch(attr: TokenStream, body: TokenStream) -> TokenStream {
    cgp_extra_macro_lib::cgp_auto_dispatch(attr.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
