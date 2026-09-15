//! `delegate_components!` supports no attributes, so it rejects any attribute it
//! finds — on the table, on a key, and, crucially, on a key nested inside a
//! `UseDelegate<new Inner { .. }>` value, which the validator must recurse into
//! rather than silently drop. It also rejects a braceless `open` header that
//! lists more than one component, since the braceless form opens exactly one.
//!
//! Three further rejections come from the body grammar rather than from
//! attributes, and each is a documented diagnostic whose wording is misleading
//! enough to be worth pinning: a statement written after a mapping, a braced path
//! group followed by more path, and a bounded generic list on a nested table.
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

#[test]
fn rejects_statement_after_mapping() {
    // Statements must lead the block. Once the parser has moved on to mappings it
    // reads `open` as a *key* type, then looks for an operator and finds the
    // component name — so the rejection is real but the message (`expected `:``)
    // blames the component rather than the misplaced statement.
    assert_macro_rejects(
        "delegate_components with a statement written after a mapping",
        || {
            cgp_macro_lib::delegate_components(quote!(
                Context {
                    BarComponent: Bar,

                    open FooComponent;

                    @FooComponent.String: Foo,
                }
            ))
        },
    );
}

#[test]
fn rejects_braced_path_group_followed_by_more_path() {
    // A braced group holds whole path *tails* and therefore terminates the path,
    // unlike a bracketed group, which holds alternatives for one segment and may
    // be followed by more. The trailing `.bool` is left where the mapping's
    // operator should be, so this too reports `expected `:``.
    assert_macro_rejects(
        "delegate_components with a braced path group followed by more path",
        || {
            cgp_macro_lib::delegate_components(quote!(
                Context {
                    open FooComponent;

                    @FooComponent.{String, u32}.bool: Foo,
                }
            ))
        },
    );
}

#[test]
fn rejects_bounded_generics_on_inner_table() {
    // A nested table's name takes a bound-free generic list; a bound belongs on
    // the entry's own generics instead. The value parser tries the nested-table
    // form speculatively and falls back to reading the whole value as a plain
    // type when it fails, so the message is an unrelated `expected `,`` rather
    // than anything about generics.
    assert_macro_rejects(
        "delegate_components with a bounded generic list on a nested table",
        || {
            cgp_macro_lib::delegate_components(quote!(
                Context {
                    <T: Clone> BarKey<T>: UseDelegate<new BarValue<T: Clone> {
                        BazKey: BazValue<T>,
                    }>,
                }
            ))
        },
    );
}
