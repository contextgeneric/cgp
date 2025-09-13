use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, Ident, ItemTrait};

use crate::blanket_trait::derive_blanket_trait;

pub fn blanket_trait(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_ident: Ident = if attr.is_empty() {
        Ident::new("__Context__", Span::call_site())
    } else {
        parse2(attr)?
    };

    let mut item_trait: ItemTrait = parse2(body)?;

    let item_impl = derive_blanket_trait(&context_ident, &mut item_trait)?;

    let out = quote! {
        #item_trait

        #item_impl
    };

    Ok(out)
}
