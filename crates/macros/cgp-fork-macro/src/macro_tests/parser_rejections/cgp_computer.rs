//! `#[cgp_computer]` rejects a receiver, a mutable context-field argument, and a
//! `#[field]` / `#[implicit]` attribute that takes arguments.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_computer.md.

use quote::quote;

use super::{assert_macro_rejects, assert_macro_rejects_with};

#[test]
fn rejects_receiver() {
    assert_macro_rejects("cgp_computer with a self receiver", || {
        crate::extra_macro_lib::cgp_computer(
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
        crate::extra_macro_lib::cgp_computer(
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
fn rejects_one_argument_result_alias() {
    assert_macro_rejects_with(
        "cgp_computer returning a one-argument Result alias",
        "A `Result` return type must be written as `Result<T, E>`, naming its error type",
        || {
            crate::extra_macro_lib::cgp_computer(
                quote!(),
                quote!(
                    fn parse(value: String) -> Result<u64> {
                        todo!()
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_impl_trait_parameter() {
    assert_macro_rejects_with(
        "cgp_computer with an impl Trait parameter",
        "Computer function parameters cannot use `impl Trait`; declare a generic parameter instead",
        || {
            crate::extra_macro_lib::cgp_computer(
                quote!(),
                quote!(
                    fn show(values: Vec<impl core::fmt::Display>) -> usize {
                        values.len()
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_impl_trait_return() {
    assert_macro_rejects_with(
        "cgp_computer returning impl Trait",
        "Computer functions cannot return `impl Trait`",
        || {
            crate::extra_macro_lib::cgp_computer(
                quote!(),
                quote!(
                    fn show(value: u64) -> impl core::fmt::Display {
                        value
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_field_attr_with_arguments() {
    assert_macro_rejects("cgp_computer with a `#[field(...)]` attribute", || {
        crate::extra_macro_lib::cgp_computer(
            quote!(),
            quote!(
                fn area(#[field(width)] width: f64) -> f64 {
                    width
                }
            ),
        )
    });
}
