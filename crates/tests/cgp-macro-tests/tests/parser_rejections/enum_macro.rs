//! `Enum!` rejects the parts of an enum body a type-level shape has no use for:
//! variant attributes, variant visibility, discriminants, and a variant name
//! given twice. A variant's fields go through the same checks as a `Struct!`
//! body, so a rejected field is rejected inside a variant too.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/enum.md.

use quote::quote;

use super::assert_macro_rejects_with;

#[test]
fn rejects_a_variant_attribute() {
    assert_macro_rejects_with(
        "Enum! with a variant attribute",
        "unsupported attribute: default",
        || {
            cgp_macro_lib::Enum(quote!(
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
        || cgp_macro_lib::Enum(quote!(pub Circle(f64))),
    );
}

#[test]
fn rejects_a_discriminant() {
    assert_macro_rejects_with(
        "Enum! with a discriminant",
        "an `Enum!` variant has no discriminant; remove the `= …`",
        || cgp_macro_lib::Enum(quote!(A = 1, B = 2)),
    );
}

#[test]
fn rejects_a_duplicate_variant_name() {
    assert_macro_rejects_with(
        "Enum! with a duplicate variant",
        "duplicate variant `Circle`",
        || cgp_macro_lib::Enum(quote!(Circle(f64), Circle(u32))),
    );
}

#[test]
fn rejects_a_bad_field_inside_a_variant() {
    assert_macro_rejects_with(
        "Enum! with a `pub` named field",
        "a field of a type-level shape has no visibility; remove it",
        || cgp_macro_lib::Enum(quote!(Rect { pub width: f64 })),
    );

    assert_macro_rejects_with(
        "Enum! with a `pub` positional field",
        "a field of a type-level shape has no visibility; remove it",
        || cgp_macro_lib::Enum(quote!(Pair(pub u8, u16))),
    );

    assert_macro_rejects_with(
        "Enum! with a duplicate variant field",
        "duplicate field `width`",
        || {
            cgp_macro_lib::Enum(quote!(Rect {
                width: f64,
                width: f64
            }))
        },
    );

    assert_macro_rejects_with(
        "Enum! with a variant field attribute",
        "unsupported attribute: doc",
        || {
            cgp_macro_lib::Enum(quote!(Rect {
                /// The width.
                width: f64
            }))
        },
    );
}
