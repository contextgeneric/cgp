use cgp_macro_core::types::field::Symbol;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

#[allow(non_snake_case)]
/// A type-level string, used as the tag that names a struct field in CGP's field-access traits.
///
/// See [`Symbol`](https://contextgeneric.dev/docs/reference/macros/symbol).
pub fn Symbol(body: TokenStream) -> syn::Result<TokenStream> {
    let symbol: Symbol = parse2(body)?;

    Ok(symbol.to_token_stream())
}
