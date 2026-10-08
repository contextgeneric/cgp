//! `#[cgp_auto_error]` rejects an attribute argument, a trait impl, a missing
//! piece of the error definition, and a method that is not a provider function.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_error.md.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_argument() {
    assert_macro_rejects("cgp_auto_error with an attribute argument", || {
        crate::macro_lib::cgp_auto_error(
            quote!(Context),
            quote!(
                impl ProvideAppError {
                    type Error = String;
                    fn raise_error<Source>(source: Source) -> Error
                    where
                        Source: core::fmt::Display,
                    {
                        source.to_string()
                    }
                    fn wrap_error<Detail>(error: Error, detail: Detail) -> Error
                    where
                        Detail: core::fmt::Display,
                    {
                        format!("{detail}: {error}")
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_trait_impl() {
    assert_macro_rejects("cgp_auto_error on a trait impl", || {
        crate::macro_lib::cgp_auto_error(
            quote!(),
            quote!(
                impl ProvideAppError for App {
                    type Error = String;
                }
            ),
        )
    });
}

#[test]
fn rejects_missing_raise_error() {
    assert_macro_rejects("cgp_auto_error without raise_error", || {
        crate::macro_lib::cgp_auto_error(
            quote!(),
            quote!(
                impl ProvideAppError {
                    type Error = String;
                    fn wrap_error<Detail>(error: Error, detail: Detail) -> Error {
                        format!("{detail}: {error}")
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_receiver() {
    assert_macro_rejects("cgp_auto_error method with a receiver", || {
        crate::macro_lib::cgp_auto_error(
            quote!(),
            quote!(
                impl ProvideAppError {
                    type Error = String;
                    fn raise_error<Source>(&self, source: Source) -> Error {
                        source.to_string()
                    }
                    fn wrap_error<Detail>(error: Error, detail: Detail) -> Error {
                        format!("{detail}: {error}")
                    }
                }
            ),
        )
    });
}
