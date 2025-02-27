use syn::parse::{Parse, ParseStream};
use syn::token::Lt;
use syn::Ident;

use crate::parse::TypeGenerics;

pub struct SimpleType {
    pub name: Ident,
    pub generics: Option<TypeGenerics>,
}

impl Parse for SimpleType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let generics = if input.peek(Lt) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { name, generics })
    }
}
