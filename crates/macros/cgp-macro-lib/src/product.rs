use cgp_macro_core::types::product::{ProductExpr, ProductType};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

/// A type-level list and its value-level counterpart, used for the field lists of a record and for handler pipelines.
///
/// See [`Product`](https://contextgeneric.dev/docs/reference/macros/product).
#[allow(non_snake_case)]
pub fn Product(body: TokenStream) -> syn::Result<TokenStream> {
    let product_type: ProductType = parse2(body)?;

    let evaluated = product_type.eval()?;

    Ok(evaluated.to_token_stream())
}

/// A type-level list and its value-level counterpart, used for the field lists of a record and for handler pipelines.
///
/// See [`product`](https://contextgeneric.dev/docs/reference/macros/product).
pub fn product(body: TokenStream) -> syn::Result<TokenStream> {
    let product_expr: ProductExpr = parse2(body)?;

    let evaluated = product_expr.eval()?;

    Ok(evaluated.to_token_stream())
}
