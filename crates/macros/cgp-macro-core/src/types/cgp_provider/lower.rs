use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, ItemStruct};

pub struct LoweredCgpProvider {
    pub item_impl: ItemImpl,
    pub is_provider_for_impl: ItemImpl,
    pub provider_struct: Option<ItemStruct>,
}

impl ToTokens for LoweredCgpProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.item_impl.to_tokens(tokens);
        self.is_provider_for_impl.to_tokens(tokens);
        self.provider_struct.to_tokens(tokens);
    }
}
