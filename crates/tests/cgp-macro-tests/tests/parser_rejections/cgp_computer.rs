//! `#[cgp_computer]` rejects a function with a `self` receiver, since a handler
//! provider has no receiver, and a one-argument `Result<T>` alias, which its
//! syntactic `Result` detection cannot parse.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md (Tests) for these
//! failure cases, and cgp-knowledge-base/cgp/reference/macros/cgp_computer.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_self_receiver() {
    assert_macro_rejects("cgp_computer on a method with a receiver", || {
        cgp_extra_macro_lib::cgp_computer(
            quote!(),
            quote!(
                fn add(&self, b: u64) -> u64 {
                    b
                }
            ),
        )
    });
}

#[test]
fn rejects_one_argument_result_alias() {
    // Recorded as a Known issue: `Result<u64>` starts with `Result`, so the parser
    // demands a second argument and fails with "expected `,`".
    assert_macro_rejects("cgp_computer returning a one-argument Result alias", || {
        cgp_extra_macro_lib::cgp_computer(
            quote!(),
            quote!(
                fn parse(value: String) -> Result<u64> {
                    todo!()
                }
            ),
        )
    });
}
