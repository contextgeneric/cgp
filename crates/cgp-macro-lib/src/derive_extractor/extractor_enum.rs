use proc_macro2::Span;
use quote::quote;
use syn::{parse2, GenericParam, Ident, ItemEnum, Lifetime, LifetimeParam, Type, TypeParam};

use crate::derive_builder::index_to_generic_ident;
use crate::derive_extractor::{get_variant_type, type_to_variant_fields};

pub fn derive_extractor_enum(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
    is_ref: bool,
) -> syn::Result<ItemEnum> {
    let mut extractor_enum = context_enum.clone();

    extractor_enum.ident = extractor_ident.clone();

    let generics = &mut extractor_enum.generics;

    if is_ref {
        generics.params.push(GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: Lifetime::new("'__a__", Span::call_site()),
            bounds: Default::default(),
            colon_token: Default::default(),
        }));
    }

    for (i, variant) in extractor_enum.variants.iter_mut().enumerate() {
        let generic_param_name = index_to_generic_ident(i);

        let generic_param: TypeParam = parse2(quote! {
            #generic_param_name : MapType
        })?;

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = get_variant_type(variant)?;

        let mapped_type: Type = if is_ref {
            parse2(quote! {
                <#generic_param_name as MapType>::Mapped< &'__a__ #field_type >
            })?
        } else {
            parse2(quote! {
                <#generic_param_name as MapType>::Mapped<#field_type>
            })?
        };

        variant.fields = type_to_variant_fields(&mapped_type);
    }

    Ok(extractor_enum)
}
