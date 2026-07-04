use proc_macro2::Span;
use syn::{GenericParam, Ident, ItemEnum, Lifetime, LifetimeParam, Type, TypeParam};

use crate::exports::{MapType, MapTypeRef};
use crate::parse_internal;
use crate::types::cgp_data::{get_variant_type, index_to_generic_ident, type_to_variant_fields};

/// Build the owned `__Partial{Name}` enum: a clone of the input enum that gains
/// one `__F{i}__: MapType` parameter per variant, wrapping each payload so a
/// variant can be present (`IsPresent`) or ruled out (`IsVoid`).
pub fn derive_extractor_enum(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemEnum> {
    let mut extractor_enum = context_enum.clone();

    extractor_enum.attrs.clear();

    extractor_enum.ident = extractor_ident.clone();

    let generics = &mut extractor_enum.generics;

    for (i, variant) in extractor_enum.variants.iter_mut().enumerate() {
        let generic_param_name = index_to_generic_ident(i);

        let generic_param: TypeParam = parse_internal! {
            #generic_param_name : #MapType
        };

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = get_variant_type(variant)?;

        let mapped_type: Type = parse_internal! {
            <#generic_param_name as #MapType>::Map<#field_type>
        };

        variant.fields = type_to_variant_fields(&mapped_type);
    }

    Ok(extractor_enum)
}

/// Build the borrowed `__PartialRef{Name}` enum, the ref counterpart of
/// [`derive_extractor_enum`]: it prepends a `'__a__` lifetime and an
/// `__R__: MapTypeRef` parameter that selects a shared or mutable borrow of each
/// payload, and bounds the enum's own type parameters by `'__a__`.
pub fn derive_extractor_enum_ref(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemEnum> {
    let mut extractor_enum = context_enum.clone();

    extractor_enum.attrs.clear();

    extractor_enum.ident = extractor_ident.clone();

    // A variantless enum borrows nothing, so the `'__a__` lifetime and the
    // `__R__: MapTypeRef` selector would be unused parameters (`E0392`). Emit the
    // borrowed partial enum as a bare empty enum instead, and the impls that
    // reference it drop the same two parameters in lockstep.
    if extractor_enum.variants.is_empty() {
        return Ok(extractor_enum);
    }

    let generics = &mut extractor_enum.generics;

    for param in generics.params.iter_mut() {
        match param {
            GenericParam::Type(param) => {
                param.bounds.push(parse_internal! {
                    '__a__
                });
            }
            GenericParam::Lifetime(param) => {
                param.bounds.push(parse_internal! {
                    '__a__
                });
            }
            _ => {}
        }
    }

    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: Lifetime::new("'__a__", Span::call_site()),
            bounds: Default::default(),
            colon_token: Default::default(),
        }),
    );

    generics.params.insert(
        1,
        parse_internal! {
            __R__: #MapTypeRef
        },
    );

    for (i, variant) in extractor_enum.variants.iter_mut().enumerate() {
        let generic_param_name = index_to_generic_ident(i);

        let generic_param: TypeParam = parse_internal! {
            #generic_param_name : #MapType
        };

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = get_variant_type(variant)?;

        let mapped_type: Type = parse_internal! {
            <#generic_param_name as #MapType>::Map<
                <__R__ as #MapTypeRef>::Map<'__a__ , #field_type >
            >
        };

        variant.fields = type_to_variant_fields(&mapped_type);
    }

    Ok(extractor_enum)
}
