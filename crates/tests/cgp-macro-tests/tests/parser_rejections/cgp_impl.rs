//! `#[cgp_impl]` rejects inputs it cannot lower into a provider: an attribute
//! with no provider name, a `#[cgp_impl(Self)]` block missing its `for` clause,
//! and a non-`impl` item.
//!
//! See docs/implementation/entrypoints/cgp_impl.md (Tests) for these failure
//! cases, and docs/reference/macros/cgp_impl.md for the user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_missing_provider_name() {
    // The attribute must name the provider type; an empty argument list has
    // nothing to place in the generated provider impl's `Self` position.
    assert_macro_rejects("cgp_impl with no provider name", || {
        cgp_macro_lib::cgp_impl(
            quote!(),
            quote!(
                impl Greeter {
                    fn greet(&self) {}
                }
            ),
        )
    });
}

#[test]
fn rejects_self_provider_without_for_clause() {
    // The `#[cgp_impl(Self)]` passthrough emits the block as a direct
    // consumer-trait impl, which needs an explicit `for Context`; without it
    // there is no concrete context to implement the trait on.
    assert_macro_rejects("cgp_impl(Self) without a for clause", || {
        cgp_macro_lib::cgp_impl(
            quote!(Self),
            quote!(
                impl CanGreet {
                    fn greet(&self) {}
                }
            ),
        )
    });
}

#[test]
fn rejects_non_impl_item() {
    // `#[cgp_impl]` only applies to an `impl` block, so a struct fails at the
    // `ItemImpl` parse before any lowering runs.
    assert_macro_rejects("cgp_impl on a struct", || {
        cgp_macro_lib::cgp_impl(
            quote!(GreetHello),
            quote!(
                pub struct NotAnImpl;
            ),
        )
    });
}
