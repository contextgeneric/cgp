use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, Type};

use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};

pub fn cgp_new_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let provider_impl: ItemImpl = syn::parse2(body)?;

    let component_name: Type = if !attr.is_empty() {
        syn::parse2(attr)?
    } else {
        derive_component_name_from_provider_impl(&provider_impl)?
    };

    let provider_struct = derive_provider_struct(&provider_impl)?;

    let is_provider_for_impl: ItemImpl = derive_is_provider_for(&component_name, &provider_impl)?;

    let result = quote! {
        #provider_struct
        #provider_impl
        #is_provider_for_impl
    };

    Ok(result)
}
