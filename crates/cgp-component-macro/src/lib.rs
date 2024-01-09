extern crate proc_macro;

mod helper;

#[cfg(test)]
mod tests;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::helper::derive::derive_component(attr.into(), item.into()).into()
}
