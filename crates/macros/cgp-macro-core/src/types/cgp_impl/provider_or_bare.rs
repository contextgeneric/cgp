use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemImpl;

use crate::types::cgp_provider::LoweredCgpProvider;

pub enum CgpProviderOrBareImpl {
    Bare(Box<ItemImpl>),
    Provider(Box<LoweredCgpProvider>),
}

impl ToTokens for CgpProviderOrBareImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CgpProviderOrBareImpl::Bare(item_impl) => item_impl.to_tokens(tokens),
            CgpProviderOrBareImpl::Provider(item_cgp_provider) => {
                item_cgp_provider.to_tokens(tokens)
            }
        }
    }
}
