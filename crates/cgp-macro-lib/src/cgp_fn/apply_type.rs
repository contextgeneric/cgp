use cgp_macro_core::types::attributes::UseTypeAttributes;
use syn::ItemImpl;

use crate::cgp_fn::derive_use_type_predicates;

pub fn apply_use_type_attributes_to_item_impl(
    item_impl: &mut ItemImpl,
    use_type_specs: &UseTypeAttributes,
) -> syn::Result<()> {
    use_type_specs.substitute_abstract_types_in_item_impl(item_impl);

    let predicates = derive_use_type_predicates(&use_type_specs.attributes)?;

    item_impl
        .generics
        .make_where_clause()
        .predicates
        .extend(predicates);

    Ok(())
}
