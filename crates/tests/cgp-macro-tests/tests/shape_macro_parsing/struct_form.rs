//! Which form `StructType` reads a body as: `name: Type` entries make a named
//! body, bare types make a tuple body, and an empty body is a unit body. A
//! single `:` decides it, so a path type whose `::` begins with a `:` is still a
//! positional field.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/shape.md.

use cgp_macro_core::types::shape::StructType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Fields;

#[derive(Debug, PartialEq)]
enum Form {
    Named(Vec<String>),
    Unnamed(usize),
    Unit,
}

#[track_caller]
fn form_of(body: TokenStream) -> Form {
    let struct_type: StructType = syn::parse2(body).expect("the body should parse");

    match struct_type.fields {
        Fields::Named(fields) => Form::Named(
            fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap().to_string())
                .collect(),
        ),
        Fields::Unnamed(fields) => Form::Unnamed(fields.unnamed.len()),
        Fields::Unit => Form::Unit,
    }
}

#[test]
fn name_type_entries_are_named() {
    assert_eq!(
        form_of(quote!(name: String, age: u8)),
        Form::Named(vec!["name".to_owned(), "age".to_owned()])
    );
    assert_eq!(
        form_of(quote!(r#type: u8)),
        Form::Named(vec!["r#type".to_owned()])
    );
    assert_eq!(
        form_of(quote!(value: core::marker::PhantomData<u8>,)),
        Form::Named(vec!["value".to_owned()])
    );
    // The field's `:` is followed by a type that itself starts with `::`, so the entry is named
    // even though a `::` comes right after the colon.
    assert_eq!(
        form_of(quote!(value: ::core::primitive::u8)),
        Form::Named(vec!["value".to_owned()])
    );
}

#[test]
fn bare_types_are_positional() {
    assert_eq!(form_of(quote!(u64, String)), Form::Unnamed(2));
    assert_eq!(form_of(quote!(u64,)), Form::Unnamed(1));
    assert_eq!(form_of(quote!(a::B, ::a::B)), Form::Unnamed(2));
    assert_eq!(form_of(quote!(Self, dyn Fn(u8) -> u8)), Form::Unnamed(2));
    assert_eq!(
        form_of(quote!(fn(u8), [u8; 4], (u8, u16))),
        Form::Unnamed(3)
    );
}

#[test]
fn an_empty_body_is_a_unit_body() {
    assert_eq!(form_of(quote!()), Form::Unit);
}
