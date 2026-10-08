use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::macro_core::types::shape::EnumType;

#[allow(non_snake_case)]
pub fn Enum(body: TokenStream) -> syn::Result<TokenStream> {
    let enum_type: EnumType = parse2(body)?;

    let evaluated = enum_type.eval()?;

    Ok(evaluated.to_token_stream())
}
