use cgp_macro_core::types::blanket_trait::ItemBlanketTrait;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemTrait, parse2};

pub fn blanket_trait(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_ident: Ident = if attr.is_empty() {
        Ident::new("__Context__", Span::call_site())
    } else {
        parse2(attr)?
    };

    let item_trait: ItemTrait = parse2(body)?;

    let item_blanket_impl = ItemBlanketTrait {
        context_ident,
        item_trait,
    };

    let items = item_blanket_impl.to_items()?;

    Ok(quote! {
        #( #items )*
    })
}
