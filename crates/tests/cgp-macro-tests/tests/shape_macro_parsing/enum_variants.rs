//! How `EnumType` reads each variant: a variant's delimiter is visible inside
//! the body, so braces hold named fields and parentheses positional ones, as in
//! a Rust enum, and a bare name is a unit variant.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/shape.md.

use cgp_macro_core::types::shape::EnumType;
use quote::quote;
use syn::Fields;

#[test]
fn each_variant_keeps_its_declared_shape() {
    let enum_type: EnumType = syn::parse2(quote!(
        Empty,
        Circle(u32),
        Rectangle(u32, u32),
        Triangle {
            base: u32,
            height: u32
        },
    ))
    .unwrap();

    let shapes: Vec<(String, &str, usize)> = enum_type
        .variants
        .iter()
        .map(|variant| {
            let shape = match &variant.fields {
                Fields::Named(_) => "named",
                Fields::Unnamed(_) => "unnamed",
                Fields::Unit => "unit",
            };
            (variant.ident.to_string(), shape, variant.fields.len())
        })
        .collect();

    assert_eq!(
        shapes,
        vec![
            ("Empty".to_owned(), "unit", 0),
            ("Circle".to_owned(), "unnamed", 1),
            ("Rectangle".to_owned(), "unnamed", 2),
            ("Triangle".to_owned(), "named", 2),
        ]
    );
}

#[test]
fn an_empty_body_has_no_variants() {
    let enum_type: EnumType = syn::parse2(quote!()).unwrap();

    assert!(enum_type.variants.is_empty());
}

#[test]
fn an_empty_field_list_keeps_its_delimiter_form() {
    // `V()` and `V {}` encode to the same `Nil` payload as the unit `V`, but parse as the
    // delimiter wrote them, as a Rust enum's variants do.
    let enum_type: EnumType = syn::parse2(quote!(Unit, Parens(), Braces {})).unwrap();

    let forms: Vec<&str> = enum_type
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            Fields::Named(fields) if fields.named.is_empty() => "empty named",
            Fields::Unnamed(fields) if fields.unnamed.is_empty() => "empty unnamed",
            Fields::Unit => "unit",
            _ => "non-empty",
        })
        .collect();

    assert_eq!(forms, vec!["unit", "empty unnamed", "empty named"]);
}
