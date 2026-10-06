//! `Struct!` rejects the parts of a struct body a type-level shape has no use
//! for, each with its own message: attributes (doc comments included),
//! visibility, the `_` field name, a keyword field name, a field name given
//! twice, entries that mix the named and positional forms, and a value where a
//! type belongs.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use quote::quote;

use super::assert_macro_rejects_with;

#[test]
fn rejects_a_field_attribute() {
    assert_macro_rejects_with(
        "Struct! with an attribute",
        "unsupported attribute: serde",
        || cgp_macro_lib::Struct(quote!(#[serde(rename = "n")] name: String)),
    );
}

#[test]
fn rejects_a_doc_comment() {
    assert_macro_rejects_with(
        "Struct! with a doc comment",
        "unsupported attribute: doc",
        || {
            cgp_macro_lib::Struct(quote!(
                /// The name.
                name: String
            ))
        },
    );
}

#[test]
fn rejects_visibility_on_a_named_field() {
    assert_macro_rejects_with(
        "Struct! with `pub`",
        "a field of a type-level shape has no visibility; remove it",
        || cgp_macro_lib::Struct(quote!(pub name: String)),
    );

    assert_macro_rejects_with(
        "Struct! with `pub(crate)`",
        "a field of a type-level shape has no visibility; remove it",
        || cgp_macro_lib::Struct(quote!(pub(crate) name: String)),
    );
}

#[test]
fn rejects_visibility_on_a_positional_field() {
    assert_macro_rejects_with(
        "Struct! with a `pub` positional field",
        "a field of a type-level shape has no visibility; remove it",
        || cgp_macro_lib::Struct(quote!(pub u64, String)),
    );
}

#[test]
fn rejects_an_underscore_field_name() {
    assert_macro_rejects_with(
        "Struct! with a `_` field",
        "a field of a type-level shape must be named; `_` is not allowed",
        || cgp_macro_lib::Struct(quote!(_: u8)),
    );
}

#[test]
fn rejects_a_duplicate_field_name() {
    assert_macro_rejects_with(
        "Struct! with a duplicate field",
        "duplicate field `a`",
        || cgp_macro_lib::Struct(quote!(a: u8, a: u16)),
    );

    // `r#a` and `a` produce the same `Symbol!("a")` tag.
    assert_macro_rejects_with(
        "Struct! with a duplicate raw field",
        "duplicate field `a`",
        || cgp_macro_lib::Struct(quote!(r#a: u8, a: u16)),
    );
}

#[test]
fn rejects_a_keyword_field_name() {
    // A keyword is not an identifier, so without this check the entry would be read as a
    // positional field and fail in the type parser with no mention of the name.
    assert_macro_rejects_with(
        "Struct! with a keyword field name",
        "`type` is a keyword: write the field name as `r#type`",
        || cgp_macro_lib::Struct(quote!(type: u8)),
    );

    // These four have no raw form to suggest.
    assert_macro_rejects_with(
        "Struct! with `self` as a field name",
        "`self` cannot be a field name",
        || cgp_macro_lib::Struct(quote!(self: u8)),
    );
}

#[test]
fn rejects_mixed_named_and_positional_entries() {
    let message = "cannot mix named and positional fields: write every entry as \
                   `name: Type`, or every entry as a bare type";

    assert_macro_rejects_with("Struct! named then positional", message, || {
        cgp_macro_lib::Struct(quote!(a: u8, u16))
    });

    assert_macro_rejects_with("Struct! positional then named", message, || {
        cgp_macro_lib::Struct(quote!(u8, b: u16))
    });
}

#[test]
fn rejects_a_value_where_a_type_belongs() {
    let message = "expected a type: a type-level shape lists field types, not values";

    assert_macro_rejects_with("Struct! with a named value", message, || {
        cgp_macro_lib::Struct(quote!(a: 1))
    });

    assert_macro_rejects_with("Struct! with a positional value", message, || {
        cgp_macro_lib::Struct(quote!("text", 2))
    });
}
