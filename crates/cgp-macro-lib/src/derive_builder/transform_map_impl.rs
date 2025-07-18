use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, FieldValue, Ident, ItemImpl, ItemStruct, Type};

use crate::derive_builder::{
    field_to_member, field_value_expr, index_to_generic_ident, to_generic_args,
};

pub fn derive_transform_map_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let mut generics = context_struct.generics.clone();

    let mut source_generic_args = to_generic_args(&generics)?;
    let mut target_generic_args = to_generic_args(&generics)?;

    let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

    generics.params.push(parse2(quote! {
        __Transform__
    })?);

    generics.params.push(parse2(quote! {
        __TargetMap__: MapType
    })?);

    for (i, field) in context_struct.fields.iter().enumerate() {
        let generic_param_name = index_to_generic_ident(i);

        source_generic_args.args.push(parse2(quote! {
            #generic_param_name
        })?);

        target_generic_args.args.push(parse2(quote! {
            __TargetMap__
        })?);

        generics.params.push(parse2(quote! {
            #generic_param_name: MapType
        })?);

        let field_type = &field.ty;

        let where_clause = generics.make_where_clause();

        where_clause.predicates.push(parse2(quote! {
            __Transform__: TransformMap< #generic_param_name, __TargetMap__, #field_type >
        })?);

        let field_member = field_to_member(i, field);

        builder_fields.push(field_value_expr(
            field_member.clone(),
            quote! {
            <__Transform__ as
                TransformMap< #generic_param_name, __TargetMap__, #field_type >
            > ::transform_mapped(self. #field_member) },
        )?);
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let builder_type: Type = parse2(quote! {
        #builder_ident #source_generic_args
    })?;

    let target_type: Type = parse2(quote! {
        #builder_ident #target_generic_args
    })?;

    let item_impl = parse2(quote! {
        impl #impl_generics TransformMapFields<__Transform__, __TargetMap__> for #builder_type
        #where_clause
        {
            type Output = #target_type;

            fn transform_map_fields(self) -> Self::Output {
                #builder_ident {
                    #builder_fields
                }
            }
        }
    })?;

    Ok(item_impl)
}
