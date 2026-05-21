use syn::Type;
use syn::parse::{Parse, ParseStream};

use crate::types::generics::ImplGenerics;

pub struct SingleDelegateKey {
    pub generics: ImplGenerics,
    pub ty: Type,
}

impl Parse for SingleDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let ty = input.parse()?;

        Ok(Self { generics, ty })
    }
}
