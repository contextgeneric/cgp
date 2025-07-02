use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::{
    parse2, AngleBracketedGenericArguments, Field, FieldValue, Generics, Ident, LitInt, Member,
    Type,
};

use crate::symbol::symbol_from_string;

pub fn to_generic_args(generics: &Generics) -> syn::Result<AngleBracketedGenericArguments> {
    if generics.params.is_empty() {
        parse2(quote! { < > })
    } else {
        parse2(generics.split_for_impl().1.to_token_stream())
    }
}

pub fn field_to_member(index: usize, field: &Field) -> Member {
    match &field.ident {
        Some(ident) => Member::Named(ident.clone()),
        None => Member::Unnamed(index.into()),
    }
}

pub fn field_to_tag(index: usize, field: &Field) -> syn::Result<Type> {
    match &field.ident {
        Some(ident) => Ok(symbol_from_string(&ident.to_string())),
        None => {
            let index = LitInt::new(&format!("{index}"), field.span());
            parse2(quote! { Index< #index > })
        }
    }
}

pub fn index_to_generic_ident(index: usize) -> Ident {
    Ident::new(&format!("__F{index}__"), Span::call_site())
}

pub fn field_value_expr(field_member: Member, expr: TokenStream) -> syn::Result<FieldValue> {
    Ok(FieldValue {
        attrs: Vec::new(),
        member: field_member,
        colon_token: Some(Colon(Span::call_site())),
        expr: parse2(expr)?,
    })
}
