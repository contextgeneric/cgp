use cgp_macro_core::types::sum::SumType;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

#[allow(non_snake_case)]
pub fn Sum(body: TokenStream) -> syn::Result<TokenStream> {
    let sum_type: SumType = parse2(body)?;

    let evaluated = sum_type.eval()?;

    Ok(evaluated.to_token_stream())
}
