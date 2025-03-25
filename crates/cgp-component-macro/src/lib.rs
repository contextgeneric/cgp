#![no_std]

/*!
   This crate provides the proc macros used for defining CGP components.
*/

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cgp_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::cgp_component(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::cgp_provider(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_new_provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::cgp_new_provider(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_getter(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::cgp_getter(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_auto_getter(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::cgp_auto_getter(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn delegate_components(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::delegate_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn check_components(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::check_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn cgp_preset(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::define_preset(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_type(attrs: TokenStream, body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::cgp_type(attrs.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::cgp_context(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn blanket_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::blanket_trait(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn re_export_imports(attrs: TokenStream, body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::re_export_imports(attrs.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn replace_with(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::replace_with(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
