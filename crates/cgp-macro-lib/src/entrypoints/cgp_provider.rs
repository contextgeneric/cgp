use cgp_macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse2};

pub fn cgp_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_impl: ItemImpl = parse2(body)?;
    let args: ProviderArgs = parse2(attr)?;

    let item = ItemCgpProvider { args, item_impl };

    let lowered = item.lower()?;

    let result = quote! {
        #lowered
    };

    Ok(result)
}
