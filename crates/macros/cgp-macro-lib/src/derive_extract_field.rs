use cgp_macro_core::types::cgp_data::ItemCgpVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

/// Generate an extractor that tracks in its own type which variants of an enum remain, so exhaustiveness is checked without a wildcard arm.
///
/// See [`ExtractField`](https://contextgeneric.dev/docs/reference/derives/derive_extract_field).
pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item_enum: ItemEnum = parse2(body)?;

    let variant = ItemCgpVariant { item_enum };

    let items = variant.to_extract_field_items()?;

    Ok(quote! {
        #( #items )*
    })
}
