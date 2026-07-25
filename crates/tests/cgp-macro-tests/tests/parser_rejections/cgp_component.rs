//! `#[cgp_component]` rejects inputs it cannot lower into a component: a
//! non-trait item, and a trait carrying a const generic parameter.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_component.md (Tests) for these failure
//! cases, and cgp-knowledge-base/cgp/reference/macros/cgp_component.md for the user-facing
//! semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_non_trait_item() {
    // A struct is not a trait, so the consumer-trait parser rejects it.
    assert_macro_rejects("cgp_component on a struct", || {
        cgp_macro_lib::cgp_component(
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
        cgp_macro_lib::cgp_component(
            quote!(Foo),
            quote!(
                pub trait CanFoo<const N: usize> {
                    fn foo(&self) -> usize;
                }
            ),
        )
    });
}
