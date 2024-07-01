extern crate proc_macro;

mod derive_component;
mod helper;

#[cfg(test)]
mod tests;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::derive_component::derive::derive_component(attr.into(), item.into()).into()
}
