use syn::{Ident, ItemEnum, ItemImpl};

use crate::exports::{MapType, MapTypeRef, PartialData};
use crate::functions::override_item_span;
use crate::parse_internal;
use crate::types::cgp_data::index_to_generic_ident;

/// Emit the `PartialData` impl that links a partial enum back to its original
/// enum through the `Target` associated type. `is_ref` selects the borrowed
/// partial enum, adding its `'__a__`/`__R__` parameters to the impl header.
pub fn derive_partial_data_impl_from_enum(
    context_struct: &ItemEnum,
    builder_ident: &Ident,
    is_ref: bool,
) -> syn::Result<ItemImpl> {
    let mut generics = context_struct.generics.clone();

    // The borrowed partial enum of a variantless enum carries no `'__a__`/`__R__`
    // parameters (see `derive_extractor_enum_ref`), so this impl must not declare
    // them either, or its `type_generics` would over-apply arguments to a bare
    // `__PartialRef{Name}`.
    if is_ref && !context_struct.variants.is_empty() {
        generics.params.insert(
            0,
            parse_internal! {
                '__a__
            },
        );

        generics.params.insert(
            0,
            parse_internal! {
                __R__: #MapTypeRef
            },
        );
    }

    for (index, _) in context_struct.variants.iter().enumerate() {
        let generic_param_name = index_to_generic_ident(index);

        generics.params.push(parse_internal! {
            #generic_param_name: #MapType
        });
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let context_ident = &context_struct.ident;
    let context_generics = context_struct.generics.split_for_impl().1;

    let item_impl: ItemImpl = parse_internal! {
        impl #impl_generics #PartialData
            for #builder_ident #type_generics
        #where_clause
        {
            type Target = #context_ident #context_generics;
        }
    };

    // Key the error span on the enum name the user wrote, not the whole derive.
    // See cgp-knowledge-base/cgp/implementation/README.md#spans.
    override_item_span(context_ident.span(), &item_impl)
}
