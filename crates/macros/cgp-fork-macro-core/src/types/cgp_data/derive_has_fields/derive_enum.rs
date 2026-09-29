use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, ItemImpl};

use crate::exports::{HasFields, HasFieldsRef};
use crate::functions::override_item_span;
use crate::parse_internal;
use crate::types::cgp_data::{
    derive_from_fields_for_enum, derive_to_fields_for_enum, derive_to_fields_ref_for_enum,
    variants_to_sum_type,
};

pub fn derive_has_fields_impls_from_enum(item_enum: &ItemEnum) -> syn::Result<Vec<ItemImpl>> {
    let struct_name = &item_enum.ident;
    let (impl_generics, type_generics, where_clause) = item_enum.generics.split_for_impl();

    let sum_type = variants_to_sum_type(&item_enum.variants, &TokenStream::new())?;

    let life = quote! { '__a };
    let sum_type_ref = variants_to_sum_type(&item_enum.variants, &quote! { & #life })?;

    let has_fields_impl: ItemImpl = parse_internal! {
        impl #impl_generics
            #HasFields for #struct_name #type_generics
        #where_clause
        {
            type Fields = #sum_type ;
        }
    };

    let has_fields_ref_impl: ItemImpl = parse_internal! {
        impl #impl_generics
            #HasFieldsRef for #struct_name #type_generics
        #where_clause
        {
            type FieldsRef< #life > = #sum_type_ref
            where
                Self: #life
            ;
        }
    };

    let from_fields_impl = derive_from_fields_for_enum(item_enum)?;

    let to_fields_impl = derive_to_fields_for_enum(item_enum)?;

    let to_fields_ref_impl = derive_to_fields_ref_for_enum(item_enum)?;

    // These impls are all keyed on the whole enum, so aim an error on any of
    // their headers at the enum name the user wrote rather than at the whole
    // `#[derive(...)]`. See cgp-knowledge-base/cgp/implementation/README.md#spans.
    [
        has_fields_impl,
        has_fields_ref_impl,
        from_fields_impl,
        to_fields_impl,
        to_fields_ref_impl,
    ]
    .into_iter()
    .map(|item_impl| override_item_span(struct_name.span(), &item_impl))
    .collect()
}
