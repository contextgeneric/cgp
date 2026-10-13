//! The builder and extractor companions drop another derive's helper attributes and keep the
//! documentation of each field or variant.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/derive_build_field.md and
//! cgp-knowledge-base/cgp/implementation/entrypoints/derive_extract_field.md.

use cgp_macro_test_util_lib::functions::pretty_format;
use quote::quote;

#[test]
fn test_builder_companion_drops_field_helper_attributes() {
    let output = cgp_macro_lib::derive_build_field(quote! {
        pub struct Person {
            /// The person's name.
            #[serde(rename = "full_name")]
            pub name: String,
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        !formatted.contains("serde"),
        "expected no `#[serde]` on the companion, got:\n{formatted}"
    );
    assert!(
        formatted.contains("The person's name."),
        "expected the field's documentation on the companion, got:\n{formatted}"
    );
}

#[test]
fn test_extractor_companions_drop_variant_helper_attributes() {
    let output = cgp_macro_lib::derive_extract_field(quote! {
        pub enum Mode {
            /// The mode is off.
            #[default]
            Off,
            #[serde(rename = "on")]
            On(u64),
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        !formatted.contains("#[default]") && !formatted.contains("serde"),
        "expected no helper attribute on the companions, got:\n{formatted}"
    );
    // Once on the owned companion and once on the borrowed one.
    assert_eq!(
        formatted.matches("The mode is off.").count(),
        2,
        "expected the variant's documentation on both companions, got:\n{formatted}"
    );
}
