use proc_macro2::Span;
use quote::quote;
use syn::{parse2, GenericParam, Ident, ItemStruct, Type, TypeParam};

pub fn derive_builder_struct(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemStruct> {
    let mut builder_struct = context_struct.clone();
    builder_struct.ident = builder_ident.clone();

    let generics = &mut builder_struct.generics;

    for (i, field) in builder_struct.fields.iter_mut().enumerate() {
        let generic_param_name = Ident::new(&format!("__F{}__", i), Span::call_site());

        let generic_param: TypeParam = parse2(quote! {
            #generic_param_name : MapType
        })?;

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = &field.ty;

        let mapped_type: Type = parse2(quote! {
            <#generic_param_name as MapType>::Mapped<#field_type>
        })?;

        field.ty = mapped_type;
    }

    Ok(builder_struct)
}
