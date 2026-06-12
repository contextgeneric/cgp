use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::ItemImpl;

use crate::types::empty_struct::EmptyStruct;

pub struct LoweredCgpProvider {
    pub item_impl: ItemImpl,
    pub is_provider_for_impl: ItemImpl,
    pub provider_struct: Option<EmptyStruct>,
}

impl ToTokens for LoweredCgpProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            item_impl,
            is_provider_for_impl,
            provider_struct,
        } = self;

        tokens.extend(quote! {
            #item_impl
            #is_provider_for_impl
            #provider_struct
        });
    }
}
