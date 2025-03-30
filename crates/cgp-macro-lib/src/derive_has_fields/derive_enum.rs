use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemEnum, ItemImpl};

use crate::derive_has_fields::sum::variants_to_sum_type;

pub fn derive_has_fields_impls_from_enum(item_enum: &ItemEnum) -> syn::Result<Vec<ItemImpl>> {
    let struct_name = &item_enum.ident;
    let (impl_generics, type_generics, where_clause) = item_enum.generics.split_for_impl();

    let sum_type = variants_to_sum_type(&item_enum.variants, &TokenStream::new())?;

    let has_fields_impl: ItemImpl = parse2(quote! {
        impl #impl_generics
            HasFields for #struct_name #type_generics
        #where_clause
        {
            type Fields = #sum_type ;
        }
    })?;

    Ok(vec![has_fields_impl])
}
