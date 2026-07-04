//! `#[derive(FromVariant)]` and `#[derive(CgpVariant)]` reject the enum shapes
//! their codegen cannot lower: a variant that does not carry exactly one unnamed
//! field, and (for `CgpVariant`) a non-enum item.
//!
//! See docs/implementation/entrypoints/derive_from_variant.md (Known issues,
//! Tests) and docs/implementation/entrypoints/derive_cgp_variant.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_fieldless_variant() {
    // A unit variant carries no payload, so there is no value type for the
    // constructor to wrap.
    assert_macro_rejects("derive(FromVariant) on a fieldless variant", || {
        cgp_macro_lib::derive_from_variant(quote!(
            pub enum Shape {
                Circle(Circle),
                Empty,
            }
        ))
    });
}

#[test]
fn rejects_multi_field_variant() {
    // A multi-field tuple variant has more than one payload; the constructor
    // keys a single `Value` per variant, so the shape is refused.
    assert_macro_rejects("derive(FromVariant) on a multi-field variant", || {
        cgp_macro_lib::derive_from_variant(quote!(
            pub enum Shape {
                Pair(u32, u32),
            }
        ))
    });
}

#[test]
fn rejects_struct_style_variant() {
    // A struct-style variant names its fields; the constructor expects a single
    // unnamed field.
    assert_macro_rejects("derive(FromVariant) on a struct-style variant", || {
        cgp_macro_lib::derive_from_variant(quote!(
            pub enum Shape {
                Named { x: u32 },
            }
        ))
    });
}

#[test]
fn cgp_variant_rejects_non_enum() {
    // `#[derive(CgpVariant)]` parses its input as an enum, so a struct is refused
    // at parse time — the one behavioral difference from `#[derive(CgpData)]`.
    assert_macro_rejects("derive(CgpVariant) on a struct", || {
        cgp_macro_lib::derive_cgp_variant(quote!(
            pub struct NotAnEnum {
                pub field: u32,
            }
        ))
    });
}
