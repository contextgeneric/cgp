//! `delegate_components!` supports no attributes, so it rejects any attribute it
//! finds — on the table, on a key, and, crucially, on a key nested inside a
//! `UseDelegate<new Inner { .. }>` value, which the validator must recurse into
//! rather than silently drop.
//!
//! See docs/implementation/entrypoints/delegate_components.md (Tests) for these
//! failure cases, and docs/reference/macros/delegate_components.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_on_table() {
    assert_macro_rejects("delegate_components with an attribute on the table", || {
        cgp_macro_lib::delegate_components(quote!(
            #[foo]
            Context { FooComponent: Bar }
        ))
    });
}

#[test]
fn rejects_attribute_on_key() {
    assert_macro_rejects("delegate_components with an attribute on a key", || {
        cgp_macro_lib::delegate_components(quote!(Context {
            #[foo]
            FooComponent: Bar,
        }))
    });
}

#[test]
fn rejects_attribute_on_inner_table_key() {
    // The attribute is on a key inside the nested `UseDelegate` table. The
    // validator recurses into the inner table, so this is rejected rather than
    // parsed and discarded.
    assert_macro_rejects(
        "delegate_components with an attribute on an inner-table key",
        || {
            cgp_macro_lib::delegate_components(quote!(
                Context {
                    FooComponent: UseDelegate<new Inner {
                        #[foo]
                        A: B,
                    }>,
                }
            ))
        },
    );
}
