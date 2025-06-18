use quote::quote;
use syn::{parse2, Ident, ItemImpl, ItemStruct};

use crate::derive_builder::{
    field_to_member, field_to_tag, index_to_generic_ident, to_generic_args,
};

pub fn derive_has_field_impls(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<Vec<ItemImpl>> {
    let mut item_impls = Vec::new();

    let base_generic_args = to_generic_args(&context_struct.generics)?;

    for (current_index, current_field) in context_struct.fields.iter().enumerate() {
        let field_member = field_to_member(current_index, current_field);
        let tag_type = field_to_tag(current_index, current_field)?;
        let value_type = &current_field.ty;

        let mut generics = context_struct.generics.clone();
        let mut source_generic_args = base_generic_args.args.clone();

        for (other_index, _) in context_struct.fields.iter().enumerate() {
            if other_index != current_index {
                let generic_param_name = index_to_generic_ident(other_index);

                generics.params.push(parse2(quote! {
                    #generic_param_name: MapType
                })?);

                source_generic_args.push(parse2(quote! {
                    #generic_param_name
                })?);
            } else {
                source_generic_args.push(parse2(quote! {
                    IsPresent
                })?);
            }
        }

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse2(quote! {
            impl #impl_generics HasField< #tag_type >
                for #builder_ident < #source_generic_args >
            #where_clause
            {
                type Value = #value_type;

                fn get_field(&self, tag: ::core::marker::PhantomData< #tag_type >) -> &Self::Value {
                    &self. #field_member
                }
            }
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
