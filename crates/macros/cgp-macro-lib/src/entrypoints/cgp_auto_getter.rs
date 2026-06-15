use cgp_macro_core::types::cgp_auto_getter::ItemCgpAutoGetter;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ItemTrait};

pub fn cgp_auto_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive_auto_getter] does not accept any attribute argument",
        ));
    }

    let item_trait: ItemTrait = syn::parse2(body)?;

    let item_cgp_auto_getter = ItemCgpAutoGetter::preprocess(&item_trait)?;

    let items = item_cgp_auto_getter.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
