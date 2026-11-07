use cgp_macro_core::types::cgp_data::ItemCgpVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

/// Let generic code construct an enum variant selected by a type-level tag, without naming the variant or the enum.
///
/// See [`FromVariant`](https://contextgeneric.dev/docs/reference/derives/derive_from_variant).
pub fn derive_from_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let item_enum: ItemEnum = parse2(body)?;

    let variant = ItemCgpVariant { item_enum };

    let item_impls = variant.to_from_variant_impls()?;

    Ok(quote! {
        #( #item_impls )*
    })
}
