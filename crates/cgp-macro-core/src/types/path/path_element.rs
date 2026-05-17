use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type, parse2};

use crate::types::symbol::Symbol;

pub enum PathElement {
    Type(Type),
    Symbol(Symbol),
}

impl Parse for PathElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;

        let parsed = if let Ok(ident) = parse2::<Ident>(ty.to_token_stream()) {
            Self::Symbol(Symbol { ident })
        } else {
            Self::Type(ty)
        };

        Ok(parsed)
    }
}
