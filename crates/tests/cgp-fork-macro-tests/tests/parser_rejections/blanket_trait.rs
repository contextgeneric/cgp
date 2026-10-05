//! `#[blanket_trait]` rejects trait items it cannot forward into the blanket
//! impl: a method without a default body, a constant without a default
//! expression, and an item that is not a type, method, or constant.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/blanket_trait.md (Tests) for these
//! failure cases, and cgp-knowledge-base-fork/cgp/reference/macros/blanket_trait.md for the
//! user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_method_without_default() {
    assert_macro_rejects("blanket_trait with a bodiless method", || {
        cgp_fork_macro_lib::blanket_trait(
            quote!(),
            quote!(
                pub trait FooBar: Foo {
                    fn foo_bar(&self);
                }
            ),
        )
    });
}

#[test]
fn rejects_const_without_default() {
    assert_macro_rejects("blanket_trait with a valueless const", || {
        cgp_fork_macro_lib::blanket_trait(
            quote!(),
            quote!(
                pub trait HasLimit: Foo {
                    const LIMIT: u32;
                }
            ),
        )
    });
}

#[test]
fn rejects_macro_item() {
    assert_macro_rejects("blanket_trait with a macro item", || {
        cgp_fork_macro_lib::blanket_trait(
            quote!(),
            quote!(
                pub trait FooBar: Foo {
                    some_macro!();
                }
            ),
        )
    });
}
