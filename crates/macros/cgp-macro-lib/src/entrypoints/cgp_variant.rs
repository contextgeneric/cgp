use cgp_macro_core::types::cgp_data::ItemCgpVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

pub fn derive_cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let item_enum = parse2(body)?;
    derive_cgp_variant_from_enum(item_enum)
}

pub fn derive_cgp_variant_from_enum(item_enum: ItemEnum) -> syn::Result<TokenStream> {
    let variant = ItemCgpVariant { item_enum };

    let has_fields = variant.to_has_fields_impls()?;
    let from_variant_impls = variant.to_from_variant_impls()?;
    let extract_field = variant.to_extract_field_items()?;

    Ok(quote! {
        #( #from_variant_impls )*
        #( #has_fields )*
        #( #extract_field )*
    })
}
