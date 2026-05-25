use syn::Ident;
use syn::parse::{Parse, ParseStream};

use crate::types::generics::TypeGenerics;

#[derive(Debug, Clone)]
pub struct IdentType {
    pub ident: Ident,
    pub generics: TypeGenerics,
}

impl Parse for IdentType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let generics = input.parse()?;

        Ok(Self { ident, generics })
    }
}
