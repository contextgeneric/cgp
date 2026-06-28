use cgp_macro_core::types::cgp_data::ItemCgpRecord;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

use crate::derive_build_field_from_struct;
use crate::derive_has_fields::derive_has_fields_impls_from_struct;

pub fn derive_cgp_record(body: TokenStream) -> syn::Result<TokenStream> {
    let item_struct = parse2(body)?;
    derive_cgp_record_from_struct(item_struct)
}

pub fn derive_cgp_record_from_struct(item_struct: ItemStruct) -> syn::Result<TokenStream> {
    let record = ItemCgpRecord { item_struct };

    let has_field_impls = record.to_has_field_impls()?;
    let has_fields_impls = derive_has_fields_impls_from_struct(&record.item_struct)?;
    let build_field_impls = derive_build_field_from_struct(&record.item_struct)?;

    Ok(quote! {
        #( #has_field_impls )*
        #( #has_fields_impls )*
        #build_field_impls
    })
}
