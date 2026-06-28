use cgp_macro_core::types::cgp_data::ItemCgpRecord;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

pub fn derive_build_field(body: TokenStream) -> syn::Result<TokenStream> {
    let item_struct: ItemStruct = parse2(body)?;

    let record = ItemCgpRecord { item_struct };

    let items = record.to_build_field_items()?;

    Ok(quote! {
        #( #items )*
    })
}
