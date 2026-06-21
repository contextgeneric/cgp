use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::token::Lt;

use crate::types::generics::ImplGenerics;

#[derive(Clone)]
pub struct TypeWithGenerics {
    pub ty: Type,
    pub generics: ImplGenerics,
}

impl Parse for TypeWithGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = if input.peek(Lt) {
            input.parse()?
        } else {
            ImplGenerics::default()
        };

        let ty = input.parse()?;

        Ok(Self { ty, generics })
    }
}

impl From<Type> for TypeWithGenerics {
    fn from(ty: Type) -> Self {
        Self {
            ty,
            generics: ImplGenerics::default(),
        }
    }
}
