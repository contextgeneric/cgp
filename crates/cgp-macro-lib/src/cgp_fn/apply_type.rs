use quote::ToTokens;
use syn::{ItemImpl, parse2};

use crate::cgp_fn::{UseTypeSpec, derive_use_type_predicates, substitute_abstract_types};

pub fn apply_use_type_attributes_to_item_impl(
    item_impl: &ItemImpl,
    use_type_specs: &[UseTypeSpec],
) -> syn::Result<ItemImpl> {
    let mut item_impl: ItemImpl = parse2(substitute_abstract_types(
        use_type_specs,
        item_impl.to_token_stream(),
    ))?;

    let predicates = derive_use_type_predicates(use_type_specs)?;

    item_impl
        .generics
        .make_where_clause()
        .predicates
        .extend(predicates);

    Ok(item_impl)
}
