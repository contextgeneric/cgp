use syn::parse::{Parse, ParseStream};

use crate::types::generics::ImplGenerics;
use crate::types::path::PathElement;

#[derive(Debug, Clone)]
/// A path element with its own generics.
pub struct PathElementWithGenerics {
    /// The element's generics.
    pub generics: ImplGenerics,
    /// The element.
    pub element: PathElement,
}

impl Parse for PathElementWithGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let element = input.parse()?;
        Ok(Self { generics, element })
    }
}
