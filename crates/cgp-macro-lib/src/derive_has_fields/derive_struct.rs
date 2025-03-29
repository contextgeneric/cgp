use quote::quote;
use syn::{parse2, parse_quote, ItemImpl, ItemStruct, Lifetime, Type};

use crate::derive_has_fields::from_struct_fields::derive_from_fields_for_struct;
use crate::derive_has_fields::struct_fields::extract_struct_fields;
use crate::derive_has_fields::to_struct_fields::derive_to_fields_for_struct;

pub fn derive_has_fields_impls_from_struct(item_struct: &ItemStruct) -> syn::Result<Vec<ItemImpl>> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let fields = extract_struct_fields(item_struct)?;

    let mut fields_type: Type = parse_quote! { Nil };
    let mut fields_ref_type: Type = parse_quote! { Nil };

    let life: Lifetime = parse_quote! { '__a };

    for (field_tag, field_type) in fields.iter().rev() {
        fields_type = parse2(quote! {
            Cons< Field< #field_tag, #field_type >, #fields_type >
        })?;

        fields_ref_type = parse2(quote! {
            Cons< Field< #field_tag, & #life #field_type >, #fields_ref_type >
        })?;
    }

    let has_fields_impl: ItemImpl = parse2(quote! {
        impl #impl_generics
            HasFields for #struct_name #type_generics
        #where_clause
        {
            type Fields = #fields_type ;
        }
    })?;

    let has_fields_ref_impl: ItemImpl = parse2(quote! {
        impl #impl_generics
            HasFieldsRef for #struct_name #type_generics
        #where_clause
        {
            type FieldsRef< #life > = #fields_ref_type
            where
                Self: #life
            ;
        }
    })?;

    let from_fields_impl = derive_from_fields_for_struct(item_struct)?;

    let to_fields_impl = derive_to_fields_for_struct(item_struct)?;

    Ok(vec![
        has_fields_impl,
        has_fields_ref_impl,
        from_fields_impl,
        to_fields_impl,
    ])
}
