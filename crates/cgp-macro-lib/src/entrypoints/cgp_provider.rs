use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, Type};

use crate::derive_provider::derive_is_provider_for;

pub fn cgp_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let component_name: Type = syn::parse2(attr)?;

    let provider_impl: ItemImpl = syn::parse2(body)?;

    let is_provider_for_impl: ItemImpl = derive_is_provider_for(&component_name, &provider_impl)?;

    let result = quote! {
        #provider_impl
        #is_provider_for_impl
    };

    Ok(result)
}
