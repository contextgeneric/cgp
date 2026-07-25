//! `check_components!` rejects a malformed table-level attribute set: an empty
//! `#[check_providers()]` (which would otherwise emit a check trait with no impls
//! that verifies nothing), a repeated `#[check_providers]` or `#[check_trait]`,
//! and any unrecognized attribute.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/check_components.md (Tests) for these
//! failure cases, and cgp-knowledge-base/cgp/reference/macros/check_components.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_empty_check_providers() {
    assert_macro_rejects(
        "check_components with an empty #[check_providers()]",
        || {
            cgp_macro_lib::check_components(quote!(
                #[check_providers()]
                Context { FooComponent }
            ))
        },
    );
}

#[test]
fn rejects_duplicate_check_providers() {
    assert_macro_rejects(
        "check_components with two #[check_providers] attributes",
        || {
            cgp_macro_lib::check_components(quote!(
                #[check_providers(FooProvider)]
                #[check_providers(BarProvider)]
                Context { FooComponent }
            ))
        },
    );
}

#[test]
fn rejects_duplicate_check_trait() {
    assert_macro_rejects(
        "check_components with two #[check_trait] attributes",
        || {
            cgp_macro_lib::check_components(quote!(
                #[check_trait(CheckA)]
                #[check_trait(CheckB)]
                Context { FooComponent }
            ))
        },
    );
}

#[test]
fn rejects_unknown_attribute() {
    assert_macro_rejects("check_components with an unknown table attribute", || {
        cgp_macro_lib::check_components(quote!(
            #[foo]
            Context { FooComponent }
        ))
    });
}
