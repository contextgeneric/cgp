use cgp_macro_core::types::cgp_data::ItemCgpData;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

/// Generate structural access and the incremental building and extraction operations for a struct or enum, as the umbrella over the narrower derives.
///
/// See [`CgpData`](https://contextgeneric.dev/docs/reference/derives/derive_cgp_data).
pub fn derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    let data: ItemCgpData = parse2(body)?;

    let items = data.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
