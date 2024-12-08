/*!
   This crate provides the proc macros used for defining CGP components.
*/

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cgp_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::derive_component(attr.into(), item.into()).into()
}

#[proc_macro]
pub fn delegate_components(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::delegate_components(body.into())
        .unwrap()
        .into()
}

#[proc_macro]
pub fn cgp_preset(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::define_preset(body.into())
        .unwrap()
        .into()
}

#[proc_macro]
pub fn for_each_replace(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::handle_for_each_replace(body.into())
        .unwrap()
        .into()
}

#[proc_macro]
pub fn replace_with(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::handle_replace(body.into())
        .unwrap()
        .into()
}
