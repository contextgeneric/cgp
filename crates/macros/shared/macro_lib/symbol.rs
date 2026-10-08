use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::macro_core::types::field::Symbol;

#[allow(non_snake_case)]
pub fn Symbol(body: TokenStream) -> syn::Result<TokenStream> {
    let symbol: Symbol = parse2(body)?;

    Ok(symbol.to_token_stream())
}
