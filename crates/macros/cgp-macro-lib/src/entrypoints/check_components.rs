use cgp_macro_core::types::check_components::CheckComponentsTables;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse2;

use crate::check_components::derive_check_components;

pub fn check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: CheckComponentsTables = parse2(body)?;

    let mut out = TokenStream::new();

    for spec in spec.specs {
        let (item_trait, item_impls) = derive_check_components(&spec)?;

        out.append_all(item_trait.to_token_stream());
        out.append_all(item_impls);
    }

    Ok(out)
}
