use cgp_macro_core::types::cgp_impl::{ImplArgs, ItemCgpImpl};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Error, ItemImpl, parse_quote};

use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};

pub fn derive_cgp_impl(args: ImplArgs, item_impl: ItemImpl) -> syn::Result<TokenStream> {
    let item = ItemCgpImpl { args, item_impl };

    let lowered = item.lower()?;

    if lowered.args.provider_type == parse_quote!(Self) {
        if lowered.item_impl.trait_.is_none() {
            return Err(Error::new(
                lowered.item_impl.span(),
                "Expected context type to be specified",
            ));
        }

        Ok(lowered.item_impl.to_token_stream())
    } else {
        let provider_impl = lowered.to_raw_item_impl()?;

        let component_type = match &lowered.args.component_type {
            Some(component_type) => component_type.clone(),
            None => derive_component_name_from_provider_impl(&provider_impl)?,
        };

        let is_provider_for_impl: ItemImpl =
            derive_is_provider_for(&component_type, &provider_impl)?;

        let provider_struct = if lowered.args.new.is_some() {
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
}
