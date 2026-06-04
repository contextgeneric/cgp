use cgp_macro_core::types::cgp_impl::{ImplArgs, ItemCgpImpl};
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, ItemImpl, parse_quote};

use crate::cgp_impl::derive_provider_impl;
use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};

pub fn derive_cgp_impl(args: ImplArgs, item_impl: ItemImpl) -> syn::Result<TokenStream> {
    let item = ItemCgpImpl {
        args: args.clone(),
        item_impl,
    };

    let item_impl = item.lower()?.item_impl;

    if args.provider_type == parse_quote!(Self) {
        if item_impl.trait_.is_none() {
            return Err(Error::new(
                item_impl.span(),
                "Expected context type to be specified",
            ));
        }

        Ok(quote! {
            #item_impl
        })
    } else {
        let (_context_type, provider_impl) = derive_provider_impl(&args.provider_type, item_impl)?;

        let component_type = match &args.component_type {
            Some(component_type) => component_type.clone(),
            None => derive_component_name_from_provider_impl(&provider_impl)?,
        };

        let is_provider_for_impl: ItemImpl =
            derive_is_provider_for(&component_type, &provider_impl)?;

        let provider_struct = if args.new.is_some() {
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
