use cgp_macro_core::types::cgp_auto_error::ItemCgpAutoError;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ItemImpl};

pub fn cgp_auto_error(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[cgp_auto_error] does not accept any attribute argument",
        ));
    }

    let item_impl: ItemImpl = syn::parse2(body)?;
    let item = ItemCgpAutoError::parse(&item_impl)?;
    let items = item.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
