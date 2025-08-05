use proc_macro2::Span;
use quote::quote;
use syn::{parse2, GenericParam, Ident, ItemEnum, Lifetime, LifetimeParam, Type, TypeParam};

use crate::derive_builder::index_to_generic_ident;
use crate::derive_extractor::{get_variant_type, type_to_variant_fields};

pub fn derive_extractor_enum(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemEnum> {
    let mut extractor_enum = context_enum.clone();

    extractor_enum.ident = extractor_ident.clone();

    let generics = &mut extractor_enum.generics;

    for (i, variant) in extractor_enum.variants.iter_mut().enumerate() {
        let generic_param_name = index_to_generic_ident(i);

        let generic_param: TypeParam = parse2(quote! {
            #generic_param_name : MapType
        })?;

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = get_variant_type(variant)?;

        let mapped_type: Type = parse2(quote! {
            <#generic_param_name as MapType>::Map<#field_type>
        })?;

        variant.fields = type_to_variant_fields(&mapped_type);
    }

    Ok(extractor_enum)
}

pub fn derive_extractor_enum_ref(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemEnum> {
    let mut extractor_enum = context_enum.clone();

    extractor_enum.ident = extractor_ident.clone();

    let generics = &mut extractor_enum.generics;

    for param in generics.params.iter_mut() {
        match param {
            GenericParam::Type(param) => {
                param.bounds.push(parse2(quote! {
                    '__a__
                })?);
            }
            GenericParam::Lifetime(param) => {
                param.bounds.push(parse2(quote! {
                    '__a__
                })?);
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
        parse2(quote! {
            __R__: MapTypeRef
        })?,
    );

    for (i, variant) in extractor_enum.variants.iter_mut().enumerate() {
        let generic_param_name = index_to_generic_ident(i);

        let generic_param: TypeParam = parse2(quote! {
            #generic_param_name : MapType
        })?;

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = get_variant_type(variant)?;

        let mapped_type: Type = parse2(quote! {
            <#generic_param_name as MapType>::Map<
                <__R__ as MapTypeRef>::Map<'__a__ , #field_type >
            >
        })?;

        variant.fields = type_to_variant_fields(&mapped_type);
    }

    Ok(extractor_enum)
}
