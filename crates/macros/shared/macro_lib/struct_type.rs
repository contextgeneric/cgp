use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::macro_core::types::shape::StructType;

#[allow(non_snake_case)]
pub fn Struct(body: TokenStream) -> syn::Result<TokenStream> {
    let struct_type: StructType = parse2(body)?;

    let evaluated = struct_type.eval()?;

    Ok(evaluated.to_token_stream())
}
