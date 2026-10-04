//! `#[cgp_auto_dispatch]` rejects dispatch traits it cannot lower: a non-method
//! trait item, a method without a `self` receiver, and a method with a type
//! generic parameter, which the blanket impl could only bound with a quantified
//! constraint Rust does not have. Associated consts and const generic methods are
//! rejected the same way as associated types and type generic methods. Each case
//! pins the rejection's message.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md (Tests) for
//! these failure cases, and cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md for
//! the user-facing semantics.

use quote::quote;

use super::assert_macro_rejects_with;

#[test]
fn rejects_associated_type() {
    assert_macro_rejects_with(
        "cgp_auto_dispatch with an associated type",
        "Only function items are allowed in a dispatch trait",
        || {
            cgp_extra_macro_lib::cgp_auto_dispatch(
                quote!(),
                quote!(
                    pub trait HasArea {
                        type Area;

                        fn area(&self) -> f64;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_method_without_receiver() {
    assert_macro_rejects_with(
        "cgp_auto_dispatch on a method without self",
        "Dispatcher method must have a self argument",
        || {
            cgp_extra_macro_lib::cgp_auto_dispatch(
                quote!(),
                quote!(
                    pub trait HasArea {
                        fn unit_area() -> f64;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_generic_method() {
    assert_macro_rejects_with(
        "cgp_auto_dispatch on a generic method",
        "Dispatch trait methods cannot contain non-lifetime generic parameters due to the lack of quantified constraints in Rust",
        || {
            cgp_extra_macro_lib::cgp_auto_dispatch(
                quote!(),
                quote!(
                    pub trait CanScale {
                        fn scale<T: Into<f64>>(&mut self, factor: T);
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_associated_const() {
    assert_macro_rejects_with(
        "cgp_auto_dispatch with an associated const",
        "Only function items are allowed in a dispatch trait",
        || {
            cgp_extra_macro_lib::cgp_auto_dispatch(
                quote!(),
                quote!(
                    pub trait HasArea {
                        const SIDES: u8;

                        fn area(&self) -> f64;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_const_generic_method() {
    assert_macro_rejects_with(
        "cgp_auto_dispatch on a const-generic method",
        "Dispatch trait methods cannot contain non-lifetime generic parameters due to the lack of quantified constraints in Rust",
        || {
            cgp_extra_macro_lib::cgp_auto_dispatch(
                quote!(),
                quote!(
                    pub trait CanScale {
                        fn scale<const N: usize>(&mut self);
                    }
                ),
            )
        },
    );
}
