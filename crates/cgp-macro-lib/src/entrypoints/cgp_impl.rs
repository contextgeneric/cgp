use cgp_macro_core::types::cgp_impl::ImplArgs;
use proc_macro2::TokenStream;
use syn::{ItemImpl, parse2};

use crate::cgp_impl::derive_cgp_impl;

pub fn cgp_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let spec: ImplArgs = parse2(attr)?;
    let item_impl: ItemImpl = parse2(body)?;

    derive_cgp_impl(spec, item_impl)
}
