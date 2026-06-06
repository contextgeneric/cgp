use cgp_macro_core::types::cgp_impl::{ImplArgs, ItemCgpImpl};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, parse2};

pub fn cgp_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let args: ImplArgs = parse2(attr)?;
    let item_impl: ItemImpl = parse2(body)?;

    let item_cgp_impl = ItemCgpImpl { args, item_impl };

    let lowered = item_cgp_impl.lower()?;
    let lowered = lowered.lower()?;

    Ok(lowered.to_token_stream())
}
