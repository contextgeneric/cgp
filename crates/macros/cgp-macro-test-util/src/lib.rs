//! This crate provides CGP's snapshot-testing macros, which emit the real generated code and pin it
//! as an inline `insta` snapshot.
#![warn(missing_docs)]

use cgp_macro_test_util_lib::entrypoints;
use proc_macro::TokenStream;

/// Snapshots the expansion of `delegate_components`.
#[proc_macro]
pub fn snapshot_delegate_components(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_delegate_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_component`.
#[proc_macro]
pub fn snapshot_cgp_component(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_component(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_impl`.
#[proc_macro]
pub fn snapshot_cgp_impl(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_impl(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_provider`.
#[proc_macro]
pub fn snapshot_cgp_provider(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_provider(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_new_provider`.
#[proc_macro]
pub fn snapshot_cgp_new_provider(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_new_provider(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_fn`.
#[proc_macro]
pub fn snapshot_cgp_fn(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_fn(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_auto_getter`.
#[proc_macro]
pub fn snapshot_cgp_auto_getter(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_auto_getter(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_getter`.
#[proc_macro]
pub fn snapshot_cgp_getter(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_getter(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_namespace`.
#[proc_macro]
pub fn snapshot_cgp_namespace(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_namespace(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_type`.
#[proc_macro]
pub fn snapshot_cgp_type(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_type(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `check_components`.
#[proc_macro]
pub fn snapshot_check_components(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_check_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `delegate_and_check_components`.
#[proc_macro]
pub fn snapshot_delegate_and_check_components(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_delegate_and_check_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `blanket_trait`.
#[proc_macro]
pub fn snapshot_blanket_trait(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_blanket_trait(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `derive_has_field`.
#[proc_macro]
pub fn snapshot_derive_has_field(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_derive_has_field(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `derive_has_fields`.
#[proc_macro]
pub fn snapshot_derive_has_fields(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_derive_has_fields(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `derive_cgp_data`.
#[proc_macro]
pub fn snapshot_derive_cgp_data(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_derive_cgp_data(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `derive_build_field`.
#[proc_macro]
pub fn snapshot_derive_build_field(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_derive_build_field(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `derive_extract_field`.
#[proc_macro]
pub fn snapshot_derive_extract_field(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_derive_extract_field(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `derive_from_variant`.
#[proc_macro]
pub fn snapshot_derive_from_variant(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_derive_from_variant(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_computer`.
#[proc_macro]
pub fn snapshot_cgp_computer(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_computer(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_producer`.
#[proc_macro]
pub fn snapshot_cgp_producer(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_producer(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Snapshots the expansion of `cgp_auto_dispatch`.
#[proc_macro]
pub fn snapshot_cgp_auto_dispatch(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_cgp_auto_dispatch(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
