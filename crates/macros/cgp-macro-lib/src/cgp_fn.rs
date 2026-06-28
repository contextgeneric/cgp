use cgp_macro_core::types::cgp_fn::ItemCgpFn;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, parse2};

pub fn cgp_fn(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let ident: Option<Ident> = parse2(attr)?;

    let item_cgp_fn = ItemCgpFn { ident, item_fn };

    let items = item_cgp_fn.preprocess()?.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
