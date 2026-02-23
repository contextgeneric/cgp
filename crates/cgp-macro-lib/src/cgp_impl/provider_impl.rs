use quote::ToTokens;
use syn::{ItemImpl, Type, parse_quote, parse2};

use crate::cgp_impl::transform_impl_trait;

pub fn derive_provider_impl(
    provider_type: &Type,
    mut item_impl: ItemImpl,
) -> syn::Result<(Type, ItemImpl)> {
    match &item_impl.trait_ {
        Some((_, path, _)) => {
            let consumer_trait_path = parse2(path.to_token_stream())?;
            let context_type = item_impl.self_ty.as_ref();
            let item_trait = transform_impl_trait(
                &item_impl,
                &consumer_trait_path,
                provider_type,
                context_type,
            )?;

            Ok((context_type.clone(), item_trait))
        }
        None => {
            let consumer_trait_path = parse2(item_impl.self_ty.to_token_stream())?;
            let context_type = parse_quote! { __Context__ };

            item_impl
                .generics
                .params
                .insert(0, parse_quote! { __Context__ });

            let item_trait = transform_impl_trait(
                &item_impl,
                &consumer_trait_path,
                provider_type,
                &context_type,
            )?;

            Ok((context_type, item_trait))
        }
    }
}
