extern crate proc_macro;

// mod delegate_components;
// mod derive_component;

// #[cfg(test)]
// mod tests;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    cgp_component_macro_lib::derive_component(attr.into(), item.into()).into()
}

#[proc_macro]
pub fn delegate_components(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::delegate_components(body.into()).into()
}

#[proc_macro]
pub fn define_components(body: TokenStream) -> TokenStream {
    cgp_component_macro_lib::define_components(body.into()).into()
}
