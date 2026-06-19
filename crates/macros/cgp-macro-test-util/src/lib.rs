use cgp_macro_test_util_lib::entrypoints;
use proc_macro::TokenStream;

#[proc_macro]
pub fn snapshot_delegate_components(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_delegate_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_component(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_component(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_impl(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_impl(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_provider(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_provider(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_new_provider(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_new_provider(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_fn(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_fn(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_auto_getter(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_auto_getter(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_getter(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_getter(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_namespace(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_namespace(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_cgp_type(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_type(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_check_components(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_check_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn snapshot_delegate_and_check_components(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_delegate_and_check_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
