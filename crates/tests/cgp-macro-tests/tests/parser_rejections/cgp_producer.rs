//! `#[cgp_producer]` rejects every signature that is not a synchronous,
//! non-generic function with no parameters: a parameter, a `self` receiver, an
//! `async` function, and a generic parameter.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_producer.md (Tests) for these
//! failure cases, and cgp-knowledge-base/cgp/reference/macros/cgp_producer.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_parameter() {
    assert_macro_rejects("cgp_producer with a parameter", || {
        cgp_extra_macro_lib::cgp_producer(
            quote!(),
            quote!(
                fn magic_number(seed: u64) -> u64 {
                    seed
                }
            ),
        )
    });
}

#[test]
fn rejects_self_receiver() {
    assert_macro_rejects("cgp_producer with a receiver", || {
        cgp_extra_macro_lib::cgp_producer(
            quote!(),
            quote!(
                fn magic_number(&self) -> u64 {
                    42
                }
            ),
        )
    });
}

#[test]
fn rejects_async() {
    assert_macro_rejects("cgp_producer on an async function", || {
        cgp_extra_macro_lib::cgp_producer(
            quote!(),
            quote!(
                async fn magic_number() -> u64 {
                    42
                }
            ),
        )
    });
}

#[test]
fn rejects_generic_parameter() {
    assert_macro_rejects("cgp_producer with a generic parameter", || {
        cgp_extra_macro_lib::cgp_producer(
            quote!(),
            quote!(
                fn default_value<T: Default>() -> T {
                    T::default()
                }
            ),
        )
    });
}
