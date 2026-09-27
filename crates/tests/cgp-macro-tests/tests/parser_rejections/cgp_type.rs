//! `#[cgp_type]` rejects any trait whose body is not exactly one plain associated
//! type: an empty trait, a trait with a method, a trait with two associated types,
//! and a generic or `where`-bounded associated type.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_type.md (Tests) for these failure
//! cases, and cgp-knowledge-base/cgp/reference/macros/cgp_type.md for the user-facing
//! semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_non_trait_item() {
    assert_macro_rejects("cgp_type on a struct", || {
        cgp_macro_lib::cgp_type(
            quote!(),
            quote!(
                pub struct NotATrait;
            ),
        )
    });
}

#[test]
fn rejects_empty_trait() {
    assert_macro_rejects("cgp_type on an empty trait", || {
        cgp_macro_lib::cgp_type(
            quote!(),
            quote!(
                pub trait HasNothing {}
            ),
        )
    });
}

#[test]
fn rejects_method_item() {
    // The single item must be an associated type, not a method.
    assert_macro_rejects("cgp_type on a trait with a method", || {
        cgp_macro_lib::cgp_type(
            quote!(),
            quote!(
                pub trait HasScalar {
                    fn scalar(&self) -> f64;
                }
            ),
        )
    });
}

#[test]
fn rejects_two_associated_types() {
    assert_macro_rejects("cgp_type on a trait with two associated types", || {
        cgp_macro_lib::cgp_type(
            quote!(),
            quote!(
                pub trait HasTypes {
                    type Left;
                    type Right;
                }
            ),
        )
    });
}

#[test]
fn rejects_generic_associated_type() {
    assert_macro_rejects("cgp_type on a generic associated type", || {
        cgp_macro_lib::cgp_type(
            quote!(),
            quote!(
                pub trait HasContainer {
                    type Container<T>;
                }
            ),
        )
    });
}

#[test]
fn rejects_associated_type_where_clause() {
    assert_macro_rejects("cgp_type on an associated type with a where clause", || {
        cgp_macro_lib::cgp_type(
            quote!(),
            quote!(
                pub trait HasScalar {
                    type Scalar
                    where
                        Self: Sized;
                }
            ),
        )
    });
}
