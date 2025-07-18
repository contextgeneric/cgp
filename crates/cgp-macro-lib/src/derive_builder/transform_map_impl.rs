use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, FieldValue, Ident, ItemImpl, ItemStruct, Type};

use crate::derive_builder::{field_to_member, field_value_expr, index_to_generic_ident, to_generic_args};

pub fn derive_transform_map_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let context_ident = &context_struct.ident;
    let mut generics = context_struct.generics.clone();

    let mut generic_args = to_generic_args(&generics)?;

    let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

    generics.params.push(parse2(quote! {
        __Transform__
    })?);

    generics.params.push(parse2(quote! {
        __TargetMap__: MapType
    })?);

    for (i, field) in context_struct.fields.iter().enumerate() {
        generic_args.args.push(parse2(quote! {
            __TargetMap__
        })?);

        let generic_param_name = index_to_generic_ident(i);

        generics.params.push(parse2(quote! {
            #generic_param_name: MapType
        })?);

        let where_clause = generics.make_where_clause();

        where_clause.predicates.push(parse2(quote! {
            __Transform__: TransformMap< #generic_param_name, __TargetMap__ >
        })?);

        let field_member = field_to_member(i, field);

        builder_fields.push(field_value_expr(
            field_member.clone(),
            quote! { __Transform__ ::transform_mapped(self. #field_member) },
        )?);
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let builder_type: Type = parse2(quote! {
        #builder_ident #generic_args
    })?;

    let context_type: Type = parse2(quote! {
        #context_ident #ty_generics
    })?;

    let item_impl = parse2(quote! {
        impl #impl_generics TransformMapFields<__Transform__, __TargetMap__> for #builder_type
        #where_clause
        {
            type Output = #context_type;

            fn transform_map_fields(self) -> Self::Output {
                #context_ident {
                    #builder_fields
                }
            }
        }
    })?;

    Ok(item_impl)
}
