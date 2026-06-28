use cgp_macro_core::types::cgp_data::ItemCgpVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

pub fn derive_cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let variant: ItemCgpVariant = parse2(body)?;

    let items = variant.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
