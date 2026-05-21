use syn::parse::{Parse, ParseStream};
use syn::token::At;

use crate::types::generics::ImplGenerics;
use crate::types::path::PathHead;

pub struct PathDelegateKey {
    pub generics: ImplGenerics,
    pub at: At,
    pub path: PathHead,
}

impl Parse for PathDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let at = input.parse()?;
        let path = input.parse()?;

        Ok(Self { generics, at, path })
    }
}
