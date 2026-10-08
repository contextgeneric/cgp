//! `#[cgp_producer]` rejects every signature that is not a synchronous,
//! non-generic function with no parameters: a parameter, a `self` receiver, an
//! `async` function, and a generic parameter, plus an `impl Trait` return type,
//! which a provider impl cannot name, and a provider name that is a path rather
//! than an identifier. Each case pins the rejection's message.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_producer.md (Tests) for these
//! failure cases, and cgp-knowledge-base-fork/cgp/reference/macros/cgp_producer.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects_with;

#[test]
fn rejects_parameter() {
    assert_macro_rejects_with(
        "cgp_producer with a parameter",
        "Producer functions cannot have parameters",
        || {
            crate::extra_macro_lib::cgp_producer(
                quote!(),
                quote!(
                    fn magic_number(seed: u64) -> u64 {
                        seed
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_self_receiver() {
    assert_macro_rejects_with(
        "cgp_producer with a receiver",
        "Producer functions cannot have parameters",
        || {
            crate::extra_macro_lib::cgp_producer(
                quote!(),
                quote!(
                    fn magic_number(&self) -> u64 {
                        42
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_async() {
    assert_macro_rejects_with(
        "cgp_producer on an async function",
        "Producer functions cannot be async",
        || {
            crate::extra_macro_lib::cgp_producer(
                quote!(),
                quote!(
                    async fn magic_number() -> u64 {
                        42
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_generic_parameter() {
    assert_macro_rejects_with(
        "cgp_producer with a generic parameter",
        "Producer functions must have empty generic parameters",
        || {
            crate::extra_macro_lib::cgp_producer(
                quote!(),
                quote!(
                    fn default_value<T: Default>() -> T {
                        T::default()
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_path_provider_name() {
    assert_macro_rejects_with(
        "cgp_producer with a path as the provider name",
        "unexpected token",
        || {
            crate::extra_macro_lib::cgp_producer(
                quote!(providers::MagicNumber),
                quote!(
                    fn magic_number() -> u64 {
                        42
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_impl_trait_return() {
    assert_macro_rejects_with(
        "cgp_producer returning impl Trait",
        "Producer functions cannot return `impl Trait`",
        || {
            crate::extra_macro_lib::cgp_producer(
                quote!(),
                quote!(
                    fn magic_number() -> impl core::fmt::Display {
                        42
                    }
                ),
            )
        },
    );
}
