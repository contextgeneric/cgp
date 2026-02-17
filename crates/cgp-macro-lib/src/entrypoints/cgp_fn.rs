use proc_macro2::TokenStream;
use syn::{Ident, ItemFn, parse2};

use crate::cgp_fn::derive_cgp_fn;
use crate::utils::to_camel_case_str;

pub fn cgp_fn(_attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let trait_ident = Ident::new(
        &to_camel_case_str(&item_fn.sig.ident.to_string()),
        item_fn.sig.ident.span(),
    );

    let derived = derive_cgp_fn(&trait_ident, item_fn)?;

    Ok(derived)
}
