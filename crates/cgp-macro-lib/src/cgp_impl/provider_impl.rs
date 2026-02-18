use quote::ToTokens;
use syn::{ItemImpl, Type, parse_quote, parse2};

use crate::cgp_impl::transform_impl_trait;

pub fn derive_provider_impl(
    provider_type: &Type,
    mut item_impl: ItemImpl,
) -> syn::Result<ItemImpl> {
    match &item_impl.trait_ {
        Some((_, path, _)) => {
            let consumer_trait_path = parse2(path.to_token_stream())?;
            let context_type = item_impl.self_ty.as_ref();
            transform_impl_trait(
                &item_impl,
                &consumer_trait_path,
                provider_type,
                context_type,
            )
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
                provider_type,
                &context_type,
            )
        }
    }
}
