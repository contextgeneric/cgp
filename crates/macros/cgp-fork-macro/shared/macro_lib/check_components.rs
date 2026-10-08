use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::macro_core::types::check_components::CheckComponentsTables;

pub fn check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let tables: CheckComponentsTables = parse2(body)?;

    let items = tables.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
