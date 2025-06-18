use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, FieldValue, GenericArgument, Ident, ItemImpl, ItemStruct, Type};

use crate::derive_builder::{
    field_to_member, field_to_tag, field_value_expr, index_to_generic_ident, to_generic_args,
};

pub fn derive_build_field_impls(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<Vec<ItemImpl>> {
    let mut item_impls = Vec::new();

    let base_generic_args = to_generic_args(&context_struct.generics)?;

    for (current_index, current_field) in context_struct.fields.iter().enumerate() {
        let value_type = &current_field.ty;

        let mut generics = context_struct.generics.clone();
        let mut source_generic_args = base_generic_args.args.clone();
        let mut output_generic_args = base_generic_args.args.clone();
        let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

        for (other_index, other_field) in context_struct.fields.iter().enumerate() {
            let field_member = field_to_member(other_index, other_field);

            if other_index != current_index {
                let generic_param_name = index_to_generic_ident(other_index);

                generics.params.push(parse2(quote! {
                    #generic_param_name: MapType
                })?);

                let generic_arg: GenericArgument = parse2(quote! { #generic_param_name })?;
                source_generic_args.push(generic_arg.clone());
                output_generic_args.push(generic_arg);

                builder_fields.push(field_value_expr(
                    field_member.clone(),
                    quote! { self. #field_member },
                )?);
            } else {
                source_generic_args.push(parse2(quote! { IsNothing })?);
                output_generic_args.push(parse2(quote! { IsPresent })?);

                builder_fields.push(field_value_expr(field_member, quote! { value })?);
            }
        }

        let source_type: Type = parse2(quote! {
            #builder_ident < #source_generic_args >
        })?;

        let output_type: Type = parse2(quote! {
            #builder_ident < #output_generic_args >
        })?;

        let tag_type = field_to_tag(current_index, current_field)?;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse2(quote! {
            impl #impl_generics BuildField< #tag_type >
                for #source_type
            #where_clause
            {
                type Value = #value_type;

                type Output = #output_type;

                fn build_field(self, _tag: ::core::marker::PhantomData< #tag_type >, value: Self::Value) -> Self::Output {
                    #builder_ident {
                        #builder_fields
                    }
                }
            }
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
