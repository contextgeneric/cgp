use cgp_fork_macro_core::types::cgp_auto_log::ItemCgpAutoLog;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ItemTrait};

pub fn cgp_auto_log(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[cgp_auto_log] does not accept any attribute argument",
        ));
    }

    let item_trait: ItemTrait = syn::parse2(body)?;
    let item = ItemCgpAutoLog::parse(&item_trait)?;
    let items = item.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
