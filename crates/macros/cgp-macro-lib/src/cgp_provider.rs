use cgp_macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse2};

/// Implement a provider trait directly, in the inside-out shape the ergonomic macros desugar to, with or without declaring the provider struct.
///
/// See [`cgp_provider`](https://contextgeneric.dev/docs/reference/macros/cgp_provider).
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
