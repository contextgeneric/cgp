use cgp_macro_core::types::path::UniPath;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

#[allow(non_snake_case)]
pub fn Path(body: TokenStream) -> syn::Result<TokenStream> {
    let unipath: UniPath = parse2(body)?;
    Ok(unipath.to_token_stream())
}
