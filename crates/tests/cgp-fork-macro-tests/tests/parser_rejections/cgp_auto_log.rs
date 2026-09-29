//! `#[cgp_auto_log]` rejects an attribute argument, a method body, a missing
//! receiver, and a trait item that is not a log method.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_log.md.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_argument() {
    assert_macro_rejects("cgp_auto_log with an attribute argument", || {
        cgp_fork_extra_macro_lib::cgp_auto_log(
            quote!(Info),
            quote!(
                pub trait CanLogHello {
                    fn log_hello(&self, name: &str);
                }
            ),
        )
    });
}

#[test]
fn rejects_method_body() {
    assert_macro_rejects("cgp_auto_log method with a body", || {
        cgp_fork_extra_macro_lib::cgp_auto_log(
            quote!(),
            quote!(
                pub trait CanLogHello {
                    fn log_hello(&self, name: &str) {}
                }
            ),
        )
    });
}

#[test]
fn rejects_missing_receiver() {
    assert_macro_rejects("cgp_auto_log method without &self", || {
        cgp_fork_extra_macro_lib::cgp_auto_log(
            quote!(),
            quote!(
                pub trait CanLogHello {
                    fn log_hello(name: &str);
                }
            ),
        )
    });
}

#[test]
fn rejects_associated_type() {
    assert_macro_rejects("cgp_auto_log associated type", || {
        cgp_fork_extra_macro_lib::cgp_auto_log(
            quote!(),
            quote!(
                pub trait CanLogHello {
                    type Name;
                    fn log_hello(&self, name: &str);
                }
            ),
        )
    });
}
