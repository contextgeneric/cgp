//! `delegate_components!` supports no attributes, so it rejects any attribute it
//! finds — on the table, on a key, and, crucially, on a key nested inside a
//! `UseDelegate<new Inner { .. }>` value, which the validator must recurse into
//! rather than silently drop. It also rejects a braceless `open` header that
//! lists more than one component, since the braceless form opens exactly one.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/delegate_components.md (Tests) for these
//! failure cases, and cgp-knowledge-base/cgp/reference/macros/delegate_components.md for the
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
fn rejects_braceless_open_with_multiple_components() {
    // The braceless `open` form opens a single component; listing several
    // without braces is rejected. The parser reads one component type after
    // `open`, then fails on the trailing `,` where it expects the `;`.
    assert_macro_rejects(
        "delegate_components with a braceless open listing multiple components",
        || {
            cgp_macro_lib::delegate_components(quote!(
                Context {
                    open FooComponent, BarComponent;

                    @FooComponent.String: Foo,
                    @BarComponent.u32: Bar,
                }
            ))
        },
    );
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
