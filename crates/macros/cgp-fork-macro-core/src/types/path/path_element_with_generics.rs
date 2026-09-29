use syn::parse::{Parse, ParseStream};

use crate::types::generics::ImplGenerics;
use crate::types::path::PathElement;

#[derive(Debug, Clone)]
pub struct PathElementWithGenerics {
    pub generics: ImplGenerics,
    pub element: PathElement,
}

impl Parse for PathElementWithGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let element = input.parse()?;
        Ok(Self { generics, element })
    }
}
