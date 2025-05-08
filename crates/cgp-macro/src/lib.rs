#![no_std]

/*!
   This crate provides the proc macros used for defining CGP components.
*/

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cgp_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_macro_lib::cgp_component(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_macro_lib::cgp_provider(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_new_provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_macro_lib::cgp_new_provider(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_getter(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_macro_lib::cgp_getter(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_auto_getter(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_macro_lib::cgp_auto_getter(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn delegate_components(body: TokenStream) -> TokenStream {
    cgp_macro_lib::delegate_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn check_components(body: TokenStream) -> TokenStream {
    cgp_macro_lib::check_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn delegate_and_check_components(body: TokenStream) -> TokenStream {
    cgp_macro_lib::delegate_and_check_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn cgp_preset(body: TokenStream) -> TokenStream {
    cgp_macro_lib::define_preset(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_type(attrs: TokenStream, body: TokenStream) -> TokenStream {
    cgp_macro_lib::cgp_type(attrs.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn cgp_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_macro_lib::cgp_context(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn blanket_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_macro_lib::blanket_trait(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn re_export_imports(attrs: TokenStream, body: TokenStream) -> TokenStream {
    cgp_macro_lib::re_export_imports(attrs.into(), body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn replace_with(body: TokenStream) -> TokenStream {
    cgp_macro_lib::replace_with(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn symbol(body: TokenStream) -> TokenStream {
    cgp_macro_lib::make_symbol(body.into()).into()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Product(body: TokenStream) -> TokenStream {
    cgp_macro_lib::make_product_type(body.into()).into()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Sum(body: TokenStream) -> TokenStream {
    cgp_macro_lib::make_sum_type(body.into()).into()
}

#[proc_macro]
pub fn product(body: TokenStream) -> TokenStream {
    cgp_macro_lib::make_product_expr(body.into()).into()
}

#[proc_macro_derive(HasField)]
pub fn derive_fields(item: TokenStream) -> TokenStream {
    cgp_macro_lib::derive_fields(item.into()).into()
}

#[proc_macro_derive(HasFields)]
pub fn derive_has_fields(item: TokenStream) -> TokenStream {
    cgp_macro_lib::derive_has_fields(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
