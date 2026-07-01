//! `#[cgp_component]` must be applied to a trait; applying it to another item
//! is rejected at parse time.
//!
//! See docs/reference/macros/cgp_component.md.

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
