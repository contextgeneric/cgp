//! `#[derive(CgpData)]` — the umbrella derive — rejects the inputs its shape
//! dispatch and variant codegen cannot lower: an item that is neither a struct
//! nor an enum, and (on the enum path) a variant that is not a single unnamed
//! field. The bad-variant rejection is inherited from the shared variant codegen
//! that `#[derive(CgpVariant)]`/`#[derive(FromVariant)]` also use, so this only
//! confirms the umbrella propagates it; the non-struct-or-enum rejection is
//! unique to `ItemCgpData`'s parser.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/derive_cgp_data.md (Known issues, Tests).

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_non_struct_or_enum() {
    // `ItemCgpData` parses only a struct or an enum; a union has no product/sum
    // field representation to derive, so it is refused at parse time.
    assert_macro_rejects("derive(CgpData) on a union", || {
        cgp_macro_lib::derive_cgp_data(quote!(
            pub union Value {
                int: u32,
                float: f32,
            }
        ))
    });
}

#[test]
fn rejects_struct_style_variant_on_enum() {
    // On the enum path the umbrella runs the same single-unnamed-field variant
    // codegen as `#[derive(CgpVariant)]`, so a struct-style variant is refused.
    assert_macro_rejects("derive(CgpData) on a struct-style variant", || {
        cgp_macro_lib::derive_cgp_data(quote!(
            pub enum Shape {
                Named { x: u32 },
            }
        ))
    });
}
