use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::token::At;

use crate::types::generics::ImplGenerics;
use crate::types::path::PathHead;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum PathHeadOrType {
    PathHead(PathHead),
    Type(ImplGenerics, Type),
}

impl Parse for PathHeadOrType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(At) {
            let _: At = input.parse()?;

            let path_head = input.parse()?;

            Ok(Self::PathHead(path_head))
        } else {
            let generics = input.parse()?;
            let ty = input.parse()?;
            Ok(Self::Type(generics, ty))
        }
    }
}
