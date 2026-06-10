use cgp_macro_core::types::cgp_impl::{ImplArgs, ItemCgpImpl};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse2};

pub fn cgp_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let args: ImplArgs = parse2(attr)?;
    let item_impl: ItemImpl = parse2(body)?;

    let item_cgp_impl = ItemCgpImpl { args, item_impl };

    let lowered = item_cgp_impl.lower()?;

    let default_impls = &lowered.default_impls;

    let bare_impls = lowered.lower()?;

    Ok(quote! {
        #bare_impls
        #(#default_impls)*
    })
}
