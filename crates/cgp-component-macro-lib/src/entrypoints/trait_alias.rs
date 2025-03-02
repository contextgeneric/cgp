use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemTrait};

use crate::trait_alias::derive_trait_alias;

pub fn trait_alias(_attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let mut item_trait: ItemTrait = parse2(body)?;

    let item_impl = derive_trait_alias(&mut item_trait)?;

    let out = quote! {
        #item_trait

        #item_impl
    };

    Ok(out)
}
