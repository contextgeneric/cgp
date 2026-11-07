use cgp_macro_core::types::shape::StructType;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

#[allow(non_snake_case)]
/// The type-level shape of a struct, written as the body of a struct declaration.
///
/// See [`Struct`](https://contextgeneric.dev/docs/reference/macros/struct).
pub fn Struct(body: TokenStream) -> syn::Result<TokenStream> {
    let struct_type: StructType = parse2(body)?;

    let evaluated = struct_type.eval()?;

    Ok(evaluated.to_token_stream())
}
