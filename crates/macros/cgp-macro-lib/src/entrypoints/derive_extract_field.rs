use cgp_macro_core::types::cgp_data::ItemCgpVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item_enum: ItemEnum = parse2(body)?;

    let variant = ItemCgpVariant { item_enum };

    let items = variant.to_extract_field_items()?;

    Ok(quote! {
        #( #items )*
    })
}
