use quote::quote;
use syn::{parse2, parse_quote, ItemImpl, ItemStruct, Lifetime};

use crate::derive_has_fields::to_fields_struct::derive_to_fields_constructor;

pub fn derive_to_fields_ref_for_struct(item_struct: &ItemStruct) -> syn::Result<ItemImpl> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let constructor = derive_to_fields_constructor(&item_struct.fields, |field_name| {
        quote! {
            ( &self . #field_name ) .into()
        }
    })?;

    let life: Lifetime = parse_quote! { '__a };

    let item_impl = parse2(quote! {
        impl #impl_generics
            ToFieldsRef for #struct_name #type_generics
        #where_clause
        {
            fn to_fields_ref< #life >(
                & #life self,
            ) -> Self::FieldsRef< #life >
            where
                Self: #life,
            {
                #constructor
            }
        }
    })?;

    Ok(item_impl)
}
