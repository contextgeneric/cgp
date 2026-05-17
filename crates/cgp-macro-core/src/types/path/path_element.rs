use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type, parse2};

use crate::traits::ToType;
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

impl ToType for PathElement {
    fn to_type(&self) -> Type {
        match self {
            Self::Type(ty) => ty.clone(),
            Self::Symbol(symbol) => symbol.to_type(),
        }
    }
}

impl ToTokens for PathElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Type(ty) => ty.to_tokens(tokens),
            Self::Symbol(symbol) => symbol.to_tokens(tokens),
        }
    }
}
