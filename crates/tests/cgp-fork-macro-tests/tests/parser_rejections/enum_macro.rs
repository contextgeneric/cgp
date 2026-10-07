//! `Enum!` rejects the parts of an enum body a type-level shape has no use for:
//! variant attributes, variant visibility, discriminants, and a variant name
//! given twice. A variant's fields go through the same checks as a `Struct!`
//! body, so a rejected field is rejected inside a variant too, with the same
//! message, and a field form that contradicts the variant's delimiter is
//! rejected with its own message.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/enum.md.

use quote::quote;

use super::assert_macro_rejects_with;

#[test]
fn rejects_a_variant_attribute() {
    assert_macro_rejects_with(
        "Enum! with a variant attribute",
        "unsupported attribute: default",
        || {
            cgp_fork_macro_lib::Enum(quote!(
                #[default]
                Empty,
                Full(u8)
            ))
        },
    );
}

#[test]
fn rejects_visibility_on_a_variant() {
    assert_macro_rejects_with(
        "Enum! with a `pub` variant",
        "an `Enum!` variant has no visibility; remove it",
        || cgp_fork_macro_lib::Enum(quote!(pub Circle(f64))),
    );
}

#[test]
fn rejects_a_discriminant() {
    assert_macro_rejects_with(
        "Enum! with a discriminant",
        "an `Enum!` variant has no discriminant; remove the `= …`",
        || cgp_fork_macro_lib::Enum(quote!(A = 1, B = 2)),
    );
}

#[test]
fn rejects_a_duplicate_variant_name() {
    assert_macro_rejects_with(
        "Enum! with a duplicate variant",
        "duplicate variant `Circle`",
        || cgp_fork_macro_lib::Enum(quote!(Circle(f64), Circle(u32))),
    );
}

#[test]
fn rejects_a_bad_field_inside_a_variant() {
    assert_macro_rejects_with(
        "Enum! with a `pub` named field",
        "a field of a type-level shape has no visibility; remove it",
        || cgp_fork_macro_lib::Enum(quote!(Rect { pub width: f64 })),
    );

    assert_macro_rejects_with(
        "Enum! with a `pub` positional field",
        "a field of a type-level shape has no visibility; remove it",
        || cgp_fork_macro_lib::Enum(quote!(Pair(pub u8, u16))),
    );

    assert_macro_rejects_with(
        "Enum! with a duplicate variant field",
        "duplicate field `width`",
        || {
            cgp_fork_macro_lib::Enum(quote!(Rect {
                width: f64,
                width: f64
            }))
        },
    );

    assert_macro_rejects_with(
        "Enum! with a variant field attribute",
        "unsupported attribute: doc",
        || {
            cgp_fork_macro_lib::Enum(quote!(Rect {
                /// The width.
                width: f64
            }))
        },
    );

    let value_message = "expected a type: a type-level shape lists field types, not values";

    assert_macro_rejects_with("Enum! with a positional value", value_message, || {
        cgp_fork_macro_lib::Enum(quote!(V(1)))
    });

    assert_macro_rejects_with("Enum! with a named value", value_message, || {
        cgp_fork_macro_lib::Enum(quote!(V { a: 1 }))
    });

    assert_macro_rejects_with(
        "Enum! with a keyword variant field name",
        "`type` is a keyword: write the field name as `r#type`",
        || cgp_fork_macro_lib::Enum(quote!(V { type: u8 })),
    );
}

#[test]
fn rejects_a_field_form_that_contradicts_the_variant_delimiter() {
    assert_macro_rejects_with(
        "Enum! with a bare type in braces",
        "a variant in braces holds named fields: write each entry as `name: Type`",
        || cgp_fork_macro_lib::Enum(quote!(V { u8 })),
    );

    assert_macro_rejects_with(
        "Enum! with a named field in parentheses",
        "a variant in parentheses holds bare types: write named fields in braces, as in \
         `Variant { name: Type }`",
        || cgp_fork_macro_lib::Enum(quote!(V(a: u8))),
    );
}
