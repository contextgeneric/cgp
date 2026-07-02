//! `#[cgp_fn]` rejects the implicit-argument shapes it cannot lower: an implicit
//! argument on a function with no `self` receiver, a `mut` binding pattern on an
//! implicit argument, and a `&mut` implicit argument that is not the sole implicit
//! (its exclusive borrow of the context would conflict with reading any other
//! field). Each is a rejection the macro makes during expansion, so it is pinned
//! by driving the entrypoint directly here rather than by a `compile_fail` test.
//!
//! See docs/implementation/entrypoints/cgp_fn.md (Tests) for these failure cases,
//! and docs/reference/attributes/implicit.md for the user-facing rules on where
//! `#[implicit]` may appear.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_implicit_arg_without_self() {
    // The value of an implicit argument is read from a field of `self`, so a
    // function that declares one without a `self` receiver has nowhere to read
    // from and is rejected.
    assert_macro_rejects("cgp_fn with an implicit argument but no self", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                pub fn rectangle_area(#[implicit] width: f64) -> f64 {
                    width
                }
            ),
        )
    });
}

#[test]
fn rejects_mut_pattern_on_implicit_arg() {
    // An implicit argument is bound to the injected field value; a `mut` binding
    // pattern is rejected so the read stays immutable (clone the value inside the
    // body to get a mutable local).
    assert_macro_rejects("cgp_fn with a `mut` implicit argument pattern", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                pub fn rectangle_area(&self, #[implicit] mut width: f64) -> f64 {
                    width
                }
            ),
        )
    });
}

#[test]
fn rejects_mutable_implicit_with_other_implicit() {
    // A `&mut` implicit reads through `get_field_mut`, borrowing the whole context
    // exclusively for the rest of the body, so it cannot share a function with any
    // other implicit read; the macro rejects the combination rather than emit an
    // impl that fails to borrow-check. (A lone `&mut` implicit, or any number of
    // purely immutable implicits, is accepted.)
    assert_macro_rejects(
        "cgp_fn with a &mut implicit alongside another implicit",
        || {
            cgp_macro_lib::cgp_fn(
                quote!(),
                quote!(
                    pub fn append_note(
                        &mut self,
                        #[implicit] buffer: &mut String,
                        #[implicit] note: &str,
                    ) {
                        buffer.push_str(note);
                    }
                ),
            )
        },
    );
}
