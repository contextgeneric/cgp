use quote::quote;
use syn::{GenericParam, Ident, ItemStruct, Type, TypeParam, parse2};

use crate::derive_builder::index_to_generic_ident;

pub fn derive_builder_struct(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemStruct> {
    let mut builder_struct = context_struct.clone();
    builder_struct.ident = builder_ident.clone();

    let generics = &mut builder_struct.generics;

    for (i, field) in builder_struct.fields.iter_mut().enumerate() {
        let generic_param_name = index_to_generic_ident(i);

        let generic_param: TypeParam = parse2(quote! {
            #generic_param_name : MapType
        })?;

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = &field.ty;

        let mapped_type: Type = parse2(quote! {
            <#generic_param_name as MapType>::Map<#field_type>
        })?;

        field.ty = mapped_type;
    }

    Ok(builder_struct)
}
