extern crate proc_macro;

mod delegate_components;
mod derive_component;

#[cfg(test)]
mod tests;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::derive_component::derive_component(attr.into(), item.into()).into()
}

#[proc_macro]
pub fn delegate_components(body: TokenStream) -> TokenStream {
    crate::delegate_components::delegate_components(body.into()).into()
}

#[proc_macro]
pub fn define_components(body: TokenStream) -> TokenStream {
    crate::delegate_components::define_components(body.into()).into()
}
