//! `#[cgp_auto_impl]` rejects an attribute argument, a method with no body, and
//! a trait item it cannot turn into a blanket provider.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_impl.md.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_argument() {
    assert_macro_rejects("cgp_auto_impl with an attribute argument", || {
        cgp_fork_macro_lib::cgp_auto_impl(
            quote!(Context),
            quote!(
                pub trait CanGreet {
                    fn greet(&self) {}
                }
            ),
        )
    });
}

#[test]
fn rejects_method_without_body() {
    // The method body is the blanket provider, so a signature with no body has
    // nothing to move onto the impl.
    assert_macro_rejects("cgp_auto_impl method without a body", || {
        cgp_fork_macro_lib::cgp_auto_impl(
            quote!(),
            quote!(
                pub trait CanGreet {
                    fn greet(&self);
                }
            ),
        )
    });
}

#[test]
fn rejects_const_without_value() {
    assert_macro_rejects("cgp_auto_impl const without a value", || {
        cgp_fork_macro_lib::cgp_auto_impl(
            quote!(),
            quote!(
                pub trait HasLimit {
                    const LIMIT: usize;
                }
            ),
        )
    });
}
