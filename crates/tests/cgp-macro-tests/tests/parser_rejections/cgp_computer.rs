//! `#[cgp_computer]` rejects a function with a `self` receiver, since a handler
//! provider has no receiver, a one-argument `Result<T>` alias, which its
//! syntactic `Result` detection cannot read, `impl Trait` in a parameter or the
//! return type, which a provider impl cannot name, a provider name that is a
//! path rather than an identifier, and a type or const parameter only the return
//! type uses, which the generated impl cannot determine. Each case pins the
//! rejection's message.
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
            cgp_macro_extra_lib::cgp_computer(
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
    // `Result<u64>` starts with `Result`, so the macro cannot tell whether it is
    // fallible without its error type, and asks for the two-argument form.
    assert_macro_rejects_with(
        "cgp_computer returning a one-argument Result alias",
        "A `Result` return type must be written as `Result<T, E>`, naming its error type",
        || {
            cgp_macro_extra_lib::cgp_computer(
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
            cgp_macro_extra_lib::cgp_computer(
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

#[test]
fn rejects_impl_trait_parameter() {
    assert_macro_rejects_with(
        "cgp_computer with an impl Trait parameter",
        "Computer function parameters cannot use `impl Trait`; declare a generic parameter instead",
        || {
            cgp_macro_extra_lib::cgp_computer(
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
            cgp_macro_extra_lib::cgp_computer(
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

/// The generated `Computer` impl's trait arguments carry only the input types, so
/// a parameter that only the return type names is constrained by nothing; the
/// macro rejects it on the parameter rather than emitting an `E0207`.
#[test]
fn rejects_output_only_type_parameter() {
    // A bound on the parameter itself, inline or in a `where` clause, names it but does not
    // determine it.
    for function in [
        quote!(
            fn parse<T: core::str::FromStr>(value: String) -> Option<T> {
                value.parse().ok()
            }
        ),
        quote!(
            fn parse<T>(value: String) -> Option<T>
            where
                T: core::str::FromStr,
            {
                value.parse().ok()
            }
        ),
    ] {
        assert_macro_rejects_with(
            "cgp_computer with a type parameter only the return type uses",
            "the parameter `T` appears in no argument type, so the generated `Computer` impl cannot determine it; take an argument whose type names `T`, or write the provider by hand as a `Computer` impl on a provider generic over `T`",
            || cgp_macro_extra_lib::cgp_computer(quote!(), function.clone()),
        );
    }
}

/// A const parameter only the return type uses is just as undetermined.
#[test]
fn rejects_output_only_const_parameter() {
    assert_macro_rejects_with(
        "cgp_computer with a const parameter only the return type uses",
        "the parameter `N` appears in no argument type, so the generated `Computer` impl cannot determine it; take an argument whose type names `N`, or write the provider by hand as a `Computer` impl on a provider generic over `N`",
        || {
            cgp_macro_extra_lib::cgp_computer(
                quote!(),
                quote!(
                    fn zeros<const N: usize>(value: u8) -> [u8; N] {
                        [value; N]
                    }
                ),
            )
        },
    );
}
