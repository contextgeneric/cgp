//! `#[cgp_computer]` rejects a receiver, a mutable context-field argument, and a
//! `#[field]` / `#[implicit]` attribute that takes arguments.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_computer.md.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_receiver() {
    assert_macro_rejects("cgp_computer with a self receiver", || {
        cgp_fork_extra_macro_lib::cgp_computer(
            quote!(),
            quote!(
                fn area(&self, width: f64) -> f64 {
                    width
                }
            ),
        )
    });
}

#[test]
fn rejects_mutable_field_argument() {
    assert_macro_rejects("cgp_computer with a mutable field argument", || {
        cgp_fork_extra_macro_lib::cgp_computer(
            quote!(),
            quote!(
                fn shout(#[field] name: &mut str) {
                    name.make_ascii_uppercase();
                }
            ),
        )
    });
}

#[test]
fn rejects_field_attr_with_arguments() {
    assert_macro_rejects("cgp_computer with a `#[field(...)]` attribute", || {
        cgp_fork_extra_macro_lib::cgp_computer(
            quote!(),
            quote!(
                fn area(#[field(width)] width: f64) -> f64 {
                    width
                }
            ),
        )
    });
}
