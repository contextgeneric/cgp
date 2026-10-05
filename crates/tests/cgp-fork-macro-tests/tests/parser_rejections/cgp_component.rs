//! `#[cgp_component]` rejects inputs it cannot lower into a component: a
//! non-trait item, and a trait carrying a const generic parameter.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_component.md (Tests) for these failure
//! cases, and cgp-knowledge-base-fork/cgp/reference/macros/cgp_component.md for the user-facing
//! semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_non_trait_item() {
    // A struct is not a trait, so the consumer-trait parser rejects it.
    assert_macro_rejects("cgp_component on a struct", || {
        cgp_fork_macro_lib::cgp_component(
            quote!(FooProvider),
            quote!(
                pub struct NotATrait;
            ),
        )
    });
}

#[test]
fn rejects_const_generic_parameter() {
    // A const value has no place in the `IsProviderFor` params tuple (a tuple of
    // types) and cannot key CGP's type-based wiring, so a const generic parameter
    // on the component is rejected rather than lowered into non-compiling code.
    assert_macro_rejects("cgp_component with a const generic parameter", || {
        cgp_fork_macro_lib::cgp_component(
            quote!(Foo),
            quote!(
                pub trait CanFoo<const N: usize> {
                    fn foo(&self) -> usize;
                }
            ),
        )
    });
}

#[test]
fn rejects_derive_promote_with_two_methods() {
    assert_macro_rejects("derive_promote on a component with two methods", || {
        cgp_fork_macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[derive_promote(PromoteAreaCalculator)]
                pub trait HasArea {
                    fn area(&self) -> f64;
                    fn perimeter(&self) -> f64;
                }
            ),
        )
    });
}

#[test]
fn rejects_duplicate_derive_promote() {
    assert_macro_rejects("duplicate derive_promote", || {
        cgp_fork_macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[derive_promote(PromoteArea)]
                #[derive_promote(PromoteOther)]
                pub trait HasArea {
                    fn area(&self) -> f64;
                }
            ),
        )
    });
}

#[test]
fn rejects_derive_promote_with_associated_type() {
    assert_macro_rejects(
        "derive_promote on a component with an associated type",
        || {
            cgp_fork_macro_lib::cgp_component(
                quote!(AreaCalculator),
                quote!(
                    #[derive_promote(PromoteAreaCalculator)]
                    pub trait HasArea {
                        type Unit;
                        fn area(&self) -> f64;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_derive_promote_on_async_method() {
    assert_macro_rejects("derive_promote on an async method", || {
        cgp_fork_macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[derive_promote(PromoteAreaCalculator)]
                pub trait HasArea {
                    async fn area(&self) -> f64;
                }
            ),
        )
    });
}
