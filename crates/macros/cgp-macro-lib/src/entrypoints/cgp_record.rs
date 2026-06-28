use cgp_macro_core::types::cgp_data::ItemCgpRecord;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

pub fn derive_cgp_record(body: TokenStream) -> syn::Result<TokenStream> {
    let item_struct = parse2(body)?;
    derive_cgp_record_from_struct(item_struct)
}

pub fn derive_cgp_record_from_struct(item_struct: ItemStruct) -> syn::Result<TokenStream> {
    let record = ItemCgpRecord { item_struct };

    let has_field_impls = record.to_has_field_impls()?;
    let has_fields_impls = record.to_has_fields_impls()?;
    let build_field_impls = record.to_build_field_items()?;

    Ok(quote! {
        #( #has_field_impls )*
        #( #has_fields_impls )*
        #( #build_field_impls )*
    })
}
