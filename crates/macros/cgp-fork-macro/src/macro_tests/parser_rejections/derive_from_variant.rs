//! `#[derive(FromVariant)]` and `#[derive(CgpVariant)]` reject the enum shapes
//! their codegen cannot lower: a variant with several fields or with named
//! fields, which has no single payload, and (for `CgpVariant`) a non-enum item.
//! A variant with no fields is accepted, with the payload `Nil`.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/derive_from_variant.md (Known issues,
//! Tests) and cgp-knowledge-base-fork/cgp/implementation/entrypoints/derive_cgp_variant.md for the
//! user-facing semantics.

use quote::quote;

use super::{assert_macro_rejects, assert_macro_rejects_with};

const VARIANT_SHAPE_MESSAGE: &str =
    "Expected variant to contain exactly one unnamed field, or no fields";

#[test]
fn rejects_multi_field_variant() {
    // A multi-field tuple variant has more than one payload; the constructor
    // keys a single `Value` per variant, so the shape is refused.
    assert_macro_rejects_with(
        "derive(FromVariant) on a multi-field variant",
        VARIANT_SHAPE_MESSAGE,
        || {
            crate::macro_lib::derive_from_variant(quote!(
                pub enum Shape {
                    Pair(u32, u32),
                }
            ))
        },
    );
}

#[test]
fn rejects_struct_style_variant() {
    // A struct-style variant names its fields; the constructor expects a single
    // unnamed field. An empty struct-style variant, `Named {}`, is accepted.
    assert_macro_rejects_with(
        "derive(FromVariant) on a struct-style variant",
        VARIANT_SHAPE_MESSAGE,
        || {
            crate::macro_lib::derive_from_variant(quote!(
                pub enum Shape {
                    Named { x: u32 },
                }
            ))
        },
    );
}

#[test]
fn extract_field_rejects_struct_style_variant() {
    // The extractor derive shares the constructor's variant-shape check.
    assert_macro_rejects_with(
        "derive(ExtractField) on a struct-style variant",
        VARIANT_SHAPE_MESSAGE,
        || {
            crate::macro_lib::derive_extract_field(quote!(
                pub enum Shape {
                    Circle(Circle),
                    Named { x: u32 },
                }
            ))
        },
    );
}

#[test]
fn cgp_variant_rejects_non_enum() {
    // `#[derive(CgpVariant)]` parses its input as an enum, so a struct is refused
    // at parse time — the one behavioral difference from `#[derive(CgpData)]`.
    assert_macro_rejects("derive(CgpVariant) on a struct", || {
        crate::macro_lib::derive_cgp_variant(quote!(
            pub struct NotAnEnum {
                pub field: u32,
            }
        ))
    });
}
