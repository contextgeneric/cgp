use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::{AngleBracketedGenericArguments, Field, FieldValue, Generics, Ident, Member, parse2};

use crate::functions::parse_internal;
use crate::types::field::{FieldName, Index, Symbol};

/// Renders a generic list as angle-bracketed arguments.
pub fn to_generic_args(generics: &Generics) -> syn::Result<AngleBracketedGenericArguments> {
    if generics.params.is_empty() {
        parse2(quote! { < > })
    } else {
        parse2(generics.split_for_impl().1.to_token_stream())
    }
}

/// The member a field is accessed by.
pub fn field_to_member(index: usize, field: &Field) -> Member {
    match &field.ident {
        Some(ident) => Member::Named(ident.clone()),
        None => Member::Unnamed(index.into()),
    }
}

/// The type-level tag a field is named by.
pub fn field_to_tag(index: usize, field: &Field) -> FieldName {
    match &field.ident {
        Some(ident) => FieldName::Ident(Symbol::from_ident(ident.clone())),
        None => FieldName::Index(Index {
            index,
            span: field.span(),
        }),
    }
}

/// The generic identifier for a field's index.
pub fn index_to_generic_ident(index: usize) -> Ident {
    Ident::new(&format!("__F{index}__"), Span::call_site())
}

/// Builds a field-value expression.
pub fn field_value_expr(field_member: Member, expr: TokenStream) -> syn::Result<FieldValue> {
    Ok(FieldValue {
        attrs: Vec::new(),
        member: field_member,
        colon_token: Some(Colon(Span::call_site())),
        expr: parse_internal(expr)?,
    })
}
