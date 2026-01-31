use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse2;

use crate::check_components::derive_check_components;
use crate::parse::CheckComponentsSpecs;

pub fn check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: CheckComponentsSpecs = parse2(body)?;

    let mut out = TokenStream::new();

    for spec in spec.specs {
        let (item_trait, item_impls) = derive_check_components(&spec)?;

        out.append_all(item_trait.to_token_stream());
        out.append_all(item_impls);
    }

    Ok(out)
}
