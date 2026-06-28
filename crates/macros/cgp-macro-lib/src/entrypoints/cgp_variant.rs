use cgp_macro_core::types::cgp_data::{ItemCgpVariant, derive_has_fields_impls_from_enum};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

use crate::derive_extract_field_from_enum;

pub fn derive_cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let item_enum = parse2(body)?;
    derive_cgp_variant_from_enum(item_enum)
}

pub fn derive_cgp_variant_from_enum(item_enum: ItemEnum) -> syn::Result<TokenStream> {
    let variant = ItemCgpVariant { item_enum };

    let has_fields = derive_has_fields_impls_from_enum(&variant.item_enum)?;
    let extract_field = derive_extract_field_from_enum(&variant.item_enum)?;
    let from_variant_impls = variant.to_from_variant_impls()?;

    Ok(quote! {
        #( #has_fields )*
        #extract_field
        #( #from_variant_impls )*
    })
}
