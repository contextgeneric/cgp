use cgp_macro_core::types::check_components::CheckComponentsTables;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

/// Assert at compile time that a context can use the components it wires, so a missing dependency fails at the assertion rather than at a distant call site.
///
/// See [`check_components`](https://contextgeneric.dev/docs/reference/macros/check_components).
pub fn check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let tables: CheckComponentsTables = parse2(body)?;

    let items = tables.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
