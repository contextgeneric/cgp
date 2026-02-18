use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemImpl;

use crate::cgp_impl::{ImplProviderSpec, derive_provider_impl};
use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};

pub fn derive_cgp_impl(spec: ImplProviderSpec, item_impl: ItemImpl) -> syn::Result<TokenStream> {
    let provider_impl = derive_provider_impl(&spec.provider_type, item_impl)?;

    let component_type = match &spec.component_type {
        Some(component_type) => component_type.clone(),
        None => derive_component_name_from_provider_impl(&provider_impl)?,
    };

    let is_provider_for_impl: ItemImpl = derive_is_provider_for(&component_type, &provider_impl)?;

    let provider_struct = if spec.new_struct {
        Some(derive_provider_struct(&provider_impl)?)
    } else {
        None
    };

    Ok(quote! {
        #provider_struct

        #provider_impl

        #is_provider_for_impl
    })
}
