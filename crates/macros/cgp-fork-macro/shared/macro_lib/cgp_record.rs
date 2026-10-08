use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::macro_core::types::cgp_data::ItemCgpRecord;

pub fn derive_cgp_record(body: TokenStream) -> syn::Result<TokenStream> {
    let record: ItemCgpRecord = parse2(body)?;

    let items = record.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
