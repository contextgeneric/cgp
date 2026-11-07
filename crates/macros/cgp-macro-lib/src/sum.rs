use cgp_macro_core::types::sum::SumType;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

#[allow(non_snake_case)]
/// A type-level sum, the dual of Product!, used for the variant lists of an extensible enum.
///
/// See [`Sum`](https://contextgeneric.dev/docs/reference/macros/sum).
pub fn Sum(body: TokenStream) -> syn::Result<TokenStream> {
    let sum_type: SumType = parse2(body)?;

    let evaluated = sum_type.eval()?;

    Ok(evaluated.to_token_stream())
}
