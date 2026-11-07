use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Ident, Type};

use crate::functions::parse_internal;
use crate::traits::ToType;
use crate::types::field::Symbol;

/// One segment of a type-level path.
#[derive(Debug, Clone)]
pub enum PathElement {
    /// A type segment.
    Type(Type),
    /// A symbol segment.
    Symbol(Symbol),
}

impl Parse for PathElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;

        let parsed = if let Ok(path_ident) = parse_internal::<Ident>(ty.to_token_stream()) {
            let path_str = path_ident.to_string();

            if let Some(path_char) = path_str.chars().next()
                && path_char.is_ascii_lowercase()
                && !is_primitive_type(&path_str)
            {
                Self::Symbol(Symbol::from_ident(path_ident))
            } else {
                Self::Type(ty)
            }
        } else {
            Self::Type(ty)
        };

        Ok(parsed)
    }
}

impl PathElement {
    /// The element's original source span. A `Symbol` keeps its parse-time span
    /// explicitly (its `ToTokens` synthesizes new `Chars`/`Symbol` tokens that
    /// would otherwise be `call_site`); a `Type` element carries its span in its
    /// own tokens. Used to point a diagnostic at the path segment the user wrote.
    pub fn span(&self) -> Span {
        match self {
            Self::Type(ty) => ty.span(),
            Self::Symbol(symbol) => symbol.span,
        }
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

/// Whether `ident` is a Rust primitive type name, which a path segment keeps as the type rather
/// than reading as a `Symbol`. Only the real names match, so `f` or `u2` is a symbol.
fn is_primitive_type(ident: &str) -> bool {
    matches!(
        ident,
        "u8" | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "f32"
            | "f64"
            | "char"
            | "bool"
            | "str"
    )
}
