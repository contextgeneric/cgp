use quote::quote;
use syn::{parse2, ItemEnum, ItemImpl};

use crate::derive_has_fields::to_fields_enum::derive_to_fields_match_arms;

pub fn derive_to_fields_ref_for_enum(item_enum: &ItemEnum) -> syn::Result<ItemImpl> {
    let struct_name = &item_enum.ident;
    let (impl_generics, type_generics, where_clause) = item_enum.generics.split_for_impl();

    let match_arms = derive_to_fields_match_arms(&item_enum.variants)?;

    let life = quote! { '__a };

    let item_impl = quote! {
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
                match self {
                    #match_arms
                }
            }
        }
    };

    parse2(item_impl)
}
