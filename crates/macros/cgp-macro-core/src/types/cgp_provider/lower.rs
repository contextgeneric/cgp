use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::ItemImpl;

use crate::types::empty_struct::EmptyStruct;

/// The lowered `#[cgp_provider]` items.
pub struct LoweredCgpProvider {
    /// The provider impl.
    pub item_impl: ItemImpl,
    /// The `IsProviderFor` impl.
    pub is_provider_for_impl: ItemImpl,
    /// The provider struct, when declared.
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
