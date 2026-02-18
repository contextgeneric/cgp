use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemImpl, parse_quote, parse2};

use crate::cgp_impl::{ImplProviderSpec, transform_impl_trait};
use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};

pub fn derive_cgp_impl(
    spec: ImplProviderSpec,
    mut item_impl: ItemImpl,
) -> syn::Result<TokenStream> {
    let provider_impl = match &item_impl.trait_ {
        Some((_, path, _)) => {
            let consumer_trait_path = parse2(path.to_token_stream())?;
            let context_type = item_impl.self_ty.as_ref();
            transform_impl_trait(
                &item_impl,
                &consumer_trait_path,
                &spec.provider_type,
                context_type,
            )?
        }
        None => {
            let consumer_trait_path = parse2(item_impl.self_ty.to_token_stream())?;
            let context_type = parse_quote! { __Context__ };

            item_impl
                .generics
                .params
                .insert(0, parse_quote! { __Context__ });

            transform_impl_trait(
                &item_impl,
                &consumer_trait_path,
                &spec.provider_type,
                &context_type,
            )?
        }
    };

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
