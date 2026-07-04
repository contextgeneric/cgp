use syn::{Ident, ItemEnum, ItemImpl, Type};

use crate::exports::{FinalizeExtract, IsVoid, MapTypeRef};
use crate::parse_internal;
use crate::types::cgp_data::to_generic_args;

/// Emit `FinalizeExtract` for the all-`IsVoid` configuration of the partial
/// enum, whose `match self {}` body type-checks because that configuration is
/// uninhabited. `is_ref` selects the borrowed enum, adding its `'__a__`/`__R__`.
pub fn derive_finalize_extract_impl(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
    is_ref: bool,
) -> syn::Result<ItemImpl> {
    let generics = {
        let mut generics = context_enum.generics.clone();

        // The borrowed partial enum of a variantless enum carries no
        // `'__a__`/`__R__` parameters (see `derive_extractor_enum_ref`), so this
        // impl must not declare them either.
        if is_ref && !context_enum.variants.is_empty() {
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

        generics
    };

    let mut generic_args = to_generic_args(&generics)?;

    for _variant in context_enum.variants.iter() {
        generic_args.args.push(parse_internal! {
            #IsVoid
        });
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let extractor_type: Type = parse_internal! {
        #extractor_ident #generic_args
    };

    let item_impl: ItemImpl = parse_internal! {
        impl #impl_generics #FinalizeExtract for #extractor_type
        #where_clause
        {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
    };

    Ok(item_impl)
}
