use cgp_macro_core::types::cgp_fn::ItemCgpFn;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, parse2};

/// Turn a plain Rust function into a trait with a single blanket implementation, so it runs on any context that supplies the fields it needs.
///
/// See [`cgp_fn`](https://contextgeneric.dev/docs/reference/macros/cgp_fn).
pub fn cgp_fn(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let ident: Option<Ident> = parse2(attr)?;

    let item_cgp_fn = ItemCgpFn { ident, item_fn };

    let items = item_cgp_fn.preprocess()?.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
