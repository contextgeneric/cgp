use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ItemTrait};

use crate::macro_core::types::cgp_auto_impl::ItemCgpAutoImpl;

pub fn cgp_auto_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[cgp_auto_impl] does not accept any attribute argument",
        ));
    }

    let item_trait: ItemTrait = syn::parse2(body)?;

    let item_cgp_auto_impl = ItemCgpAutoImpl::preprocess(&item_trait)?;

    let items = item_cgp_auto_impl.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
