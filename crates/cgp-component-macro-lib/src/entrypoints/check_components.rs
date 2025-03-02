use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse2;

use crate::check_components::derive_check_components;
use crate::parse::CheckComponents;

pub fn check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: CheckComponents = parse2(body)?;

    let (item_trait, item_impls) = derive_check_components(&spec)?;

    let mut out = item_trait.to_token_stream();
    out.append_all(item_impls);

    Ok(out)
}
