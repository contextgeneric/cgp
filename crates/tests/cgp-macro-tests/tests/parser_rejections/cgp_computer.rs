//! `#[cgp_computer]` rejects a function with a `self` receiver, since a handler
//! provider has no receiver, a one-argument `Result<T>` alias, which its
//! syntactic `Result` detection cannot parse, and a provider name that is a path
//! rather than an identifier. Each case pins the rejection's message.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md (Tests) for these
//! failure cases, and cgp-knowledge-base/cgp/reference/macros/cgp_computer.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects_with;

#[test]
fn rejects_self_receiver() {
    assert_macro_rejects_with(
        "cgp_computer on a method with a receiver",
        "Computer functions cannot have a receiver",
        || {
            cgp_extra_macro_lib::cgp_computer(
                quote!(),
                quote!(
                    fn add(&self, b: u64) -> u64 {
                        b
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_one_argument_result_alias() {
    // Recorded as a Known issue: `Result<u64>` starts with `Result`, so the parser
    // demands a second argument and fails with "expected `,`".
    assert_macro_rejects_with(
        "cgp_computer returning a one-argument Result alias",
        "expected `,`",
        || {
            cgp_extra_macro_lib::cgp_computer(
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
fn rejects_path_provider_name() {
    // The provider name is a single identifier; a path cannot name the struct
    // the macro declares.
    assert_macro_rejects_with(
        "cgp_computer with a path as the provider name",
        "unexpected token",
        || {
            cgp_extra_macro_lib::cgp_computer(
                quote!(providers::Add),
                quote!(
                    fn add(a: u64, b: u64) -> u64 {
                        a + b
                    }
                ),
            )
        },
    );
}
