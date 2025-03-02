use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, Ident, ItemTrait};

use crate::trait_alias::derive_trait_alias;

pub fn trait_alias(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_ident: Ident = if attr.is_empty() {
        Ident::new("Context", Span::call_site())
    } else {
        parse2(attr)?
    };

    let mut item_trait: ItemTrait = parse2(body)?;

    let item_impl = derive_trait_alias(&context_ident, &mut item_trait)?;

    let out = quote! {
        #item_trait

        #item_impl
    };

    Ok(out)
}
