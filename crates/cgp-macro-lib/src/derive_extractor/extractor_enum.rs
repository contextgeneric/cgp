use quote::quote;
use syn::{parse2, GenericParam, Ident, ItemEnum, Type, TypeParam};

use crate::derive_builder::{get_variant_type, index_to_generic_ident, type_to_variant_fields};

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
            <#generic_param_name as MapType>::Mapped<#field_type>
        })?;

        variant.fields = type_to_variant_fields(&mapped_type);
    }

    Ok(extractor_enum)
}
