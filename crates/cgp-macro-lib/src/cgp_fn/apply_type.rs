use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{ItemImpl, TypeParamBound, parse2};

use crate::cgp_fn::{UseTypeSpec, derive_use_type_trait_bounds, substitute_abstract_type};

pub fn apply_use_type_attributes_to_item_impl(
    item_impl: &ItemImpl,
    use_type_specs: &[UseTypeSpec],
) -> syn::Result<ItemImpl> {
    let mut item_impl: ItemImpl = parse2(substitute_abstract_type(
        &quote! { Self },
        use_type_specs,
        item_impl.to_token_stream(),
    ))?;

    let bounds = derive_use_type_trait_bounds(&quote! { Self }, use_type_specs)?;
    let bounds = Punctuated::<TypeParamBound, Plus>::from_iter(bounds);

    item_impl
        .generics
        .make_where_clause()
        .predicates
        .push(parse2(quote! {
            Self: #bounds
        })?);

    Ok(item_impl)
}
