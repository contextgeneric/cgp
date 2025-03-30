use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemImpl, ItemStruct};

use crate::derive_has_fields::from_fields_struct::derive_from_fields_for_struct;
use crate::derive_has_fields::product::item_fields_to_product_type;
use crate::derive_has_fields::to_fields_ref_struct::derive_to_fields_ref_for_struct;
use crate::derive_has_fields::to_fields_struct::derive_to_fields_for_struct;

pub fn derive_has_fields_impls_from_struct(item_struct: &ItemStruct) -> syn::Result<Vec<ItemImpl>> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let life = quote! { '__a };

    let fields_type = item_fields_to_product_type(&item_struct.fields, &TokenStream::new())?;

    let fields_ref_type = item_fields_to_product_type(&item_struct.fields, &quote! { & #life })?;

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

    let to_fields_ref_impl = derive_to_fields_ref_for_struct(item_struct)?;

    Ok(vec![
        has_fields_impl,
        has_fields_ref_impl,
        from_fields_impl,
        to_fields_impl,
        to_fields_ref_impl,
    ])
}
