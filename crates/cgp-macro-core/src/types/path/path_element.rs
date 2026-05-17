use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type, parse2};

use crate::traits::ToType;
use crate::types::symbol::Symbol;

#[derive(Debug, Clone)]
pub enum PathElement {
    Type(Type),
    Symbol(Symbol),
}

impl Parse for PathElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;

        let parsed = if let Ok(path_ident) = parse2::<Ident>(ty.to_token_stream()) {
            let path_str = path_ident.to_string();

            if let Some(path_char) = path_str.chars().next()
                && path_char.is_ascii_lowercase()
                && !is_primitive_type(&path_str)
            {
                Self::Symbol(Symbol { ident: path_ident })
            } else {
                Self::Type(ty)
            }
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

fn is_primitive_type(ident: &str) -> bool {
    if (ident.starts_with("i") || ident.starts_with("u") || ident.starts_with("f"))
        && ident[1..].chars().all(|c| c.is_numeric())
    {
        return true;
    }

    if ["char", "bool", "usize", "isize", "str"].contains(&ident) {
        return true;
    }

    false
}
