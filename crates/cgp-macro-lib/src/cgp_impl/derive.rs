use cgp_macro_core::traits::AddTypeParamBounds;
use cgp_macro_core::types::attributes::ImplAttributes;
use cgp_macro_core::types::cgp_impl::ImplArgs;
use cgp_macro_core::types::implicits::ImplicitArgFields;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, ItemImpl, Type, parse_quote};

use crate::cgp_impl::derive_provider_impl;
use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};

pub fn derive_cgp_impl(spec: ImplArgs, mut item_impl: ItemImpl) -> syn::Result<TokenStream> {
    let attributes = ImplAttributes::parse(&item_impl.attrs)?;
    item_impl.attrs = attributes.raw_attributes;

    let self_type: Type = parse_quote!(Self);

    let implicit_args = ImplicitArgFields::extract_from_impl_items(&mut item_impl.items)?;
    implicit_args.add_type_param_bounds(&self_type, &mut item_impl.generics)?;
    attributes
        .uses
        .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

    attributes.use_type.transform_item_impl(&mut item_impl)?;
    attributes
        .use_provider
        .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

    if spec.provider_type == self_type {
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
        let (_context_type, provider_impl) = derive_provider_impl(&spec.provider_type, item_impl)?;

        let component_type = match &spec.component_type {
            Some(component_type) => component_type.clone(),
            None => derive_component_name_from_provider_impl(&provider_impl)?,
        };

        let is_provider_for_impl: ItemImpl =
            derive_is_provider_for(&component_type, &provider_impl)?;

        let provider_struct = if spec.new.is_some() {
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
