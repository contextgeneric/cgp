use cgp_macro_core::types::shape::EnumType;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

#[allow(non_snake_case)]
/// The type-level shape of an enum, written as the body of an enum declaration.
///
/// See [`Enum`](https://contextgeneric.dev/docs/reference/macros/enum).
pub fn Enum(body: TokenStream) -> syn::Result<TokenStream> {
    let enum_type: EnumType = parse2(body)?;

    let evaluated = enum_type.eval()?;

    Ok(evaluated.to_token_stream())
}
