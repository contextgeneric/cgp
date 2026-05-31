use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use crate::types::field::{Index, Symbol};

pub enum FieldName {
    Ident(Symbol),
    Index(Index),
}

impl From<Ident> for FieldName {
    fn from(value: Ident) -> Self {
        Self::Ident(Symbol::new(value))
    }
}

impl From<Symbol> for FieldName {
    fn from(value: Symbol) -> Self {
        Self::Ident(value)
    }
}

impl From<Index> for FieldName {
    fn from(value: Index) -> Self {
        Self::Index(value)
    }
}

impl ToTokens for FieldName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(symbol) => symbol.to_tokens(tokens),
            Self::Index(index) => index.to_tokens(tokens),
        }
    }
}
