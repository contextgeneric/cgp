use cgp_macro_core::types::cgp_data::ItemCgpData;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

pub fn derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    let data: ItemCgpData = parse2(body)?;

    let items = data.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
