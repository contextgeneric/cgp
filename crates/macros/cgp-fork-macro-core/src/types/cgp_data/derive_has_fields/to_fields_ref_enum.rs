use quote::quote;
use syn::{ItemEnum, ItemImpl};

use crate::exports::ToFieldsRef;
use crate::functions::parse_internal;
use crate::types::cgp_data::derive_to_fields_match_arms;

pub fn derive_to_fields_ref_for_enum(item_enum: &ItemEnum) -> syn::Result<ItemImpl> {
    let struct_name = &item_enum.ident;
    let (impl_generics, type_generics, where_clause) = item_enum.generics.split_for_impl();

    let life = quote! { '__a };

    // A variantless enum is uninhabited, so match on the dereferenced place: a
    // bare `match self {}` over `&self` is non-exhaustive because a reference is
    // always inhabited, whereas the owned `to_fields`/`from_fields` match owned
    // uninhabited values and need no such handling.
    let body = if item_enum.variants.is_empty() {
        quote! { match *self {} }
    } else {
        let match_arms = derive_to_fields_match_arms(&item_enum.variants)?;
        quote! { match self { #match_arms } }
    };

    let item_impl = quote! {
        impl #impl_generics
            #ToFieldsRef for #struct_name #type_generics
        #where_clause
        {
            fn to_fields_ref< #life >(
                & #life self,
            ) -> Self::FieldsRef< #life >
            where
                Self: #life,
            {
                #body
            }
        }
    };

    parse_internal(item_impl)
}
