use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::token::At;

use crate::types::path::UniPath;

pub enum UniPathOrType {
    Type(Type),
    Path(UniPath),
}

impl Parse for UniPathOrType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(At) {
            let path = input.parse()?;
            Ok(Self::Path(path))
        } else {
            let ty = input.parse()?;
            Ok(Self::Type(ty))
        }
    }
}

impl ToTokens for UniPathOrType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            UniPathOrType::Type(ty) => ty.to_tokens(tokens),
            UniPathOrType::Path(path) => path.to_tokens(tokens),
        }
    }
}
