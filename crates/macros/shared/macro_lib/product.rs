use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::macro_core::types::product::{ProductExpr, ProductType};

#[allow(non_snake_case)]
pub fn Product(body: TokenStream) -> syn::Result<TokenStream> {
    let product_type: ProductType = parse2(body)?;

    let evaluated = product_type.eval()?;

    Ok(evaluated.to_token_stream())
}

pub fn product(body: TokenStream) -> syn::Result<TokenStream> {
    let product_expr: ProductExpr = parse2(body)?;

    let evaluated = product_expr.eval()?;

    Ok(evaluated.to_token_stream())
}
