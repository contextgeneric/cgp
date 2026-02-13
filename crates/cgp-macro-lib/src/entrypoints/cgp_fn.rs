use proc_macro2::TokenStream;
use syn::{ItemFn, parse2};

pub fn cgp_fn(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    todo!()
}
