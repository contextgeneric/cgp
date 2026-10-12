//! The getter macros reject any trait whose methods break the getter-method
//! contract. `#[cgp_getter]` and `#[cgp_auto_getter]` share the `parse_getter_fields`
//! parser, so the method-shape rejections below are driven through the
//! argument-free `cgp_auto_getter` entrypoint and hold identically for
//! `#[cgp_getter]`. A getter method must be a plain (non-const, non-async,
//! non-unsafe, non-generic) method whose first argument is a reference; a mutable
//! return requires a `&mut self` receiver; and at most one associated type is
//! allowed, only alongside exactly one method. An optional reference to a slice or a
//! trait object is rejected too, since the `Option<&T>` form would read an unsized
//! `Option<T>` field. `#[cgp_auto_getter]` additionally rejects any attribute argument,
//! since it has no provider name or keys to accept.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/cgp_getter.md (Tests) for these failure cases and
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_getter.md for the user-facing getter-method
//! rules.

use quote::quote;

use super::{assert_macro_rejects, assert_macro_rejects_with};

#[test]
fn rejects_const_getter_method() {
    assert_macro_rejects("cgp_auto_getter with a const getter method", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    const fn name(&self) -> &str;
                }
            ),
        )
    });
}

#[test]
fn rejects_async_getter_method() {
    assert_macro_rejects("cgp_auto_getter with an async getter method", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    async fn name(&self) -> &str;
                }
            ),
        )
    });
}

#[test]
fn rejects_unsafe_getter_method() {
    assert_macro_rejects("cgp_auto_getter with an unsafe getter method", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    unsafe fn name(&self) -> &str;
                }
            ),
        )
    });
}

#[test]
fn rejects_generic_getter_method() {
    // A getter method must not carry generic parameters: its field read is keyed by
    // the method name alone, with nothing to bind a method-level generic to.
    assert_macro_rejects("cgp_auto_getter with a generic getter method", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    fn name<T>(&self) -> &str;
                }
            ),
        )
    });
}

#[test]
fn rejects_by_value_self_receiver() {
    // The first argument must be a *reference* — a by-value `self` receiver cannot
    // back a getter that borrows a field out of the context.
    assert_macro_rejects("cgp_auto_getter with a by-value self receiver", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    fn name(self) -> &str;
                }
            ),
        )
    });
}

#[test]
fn rejects_mutable_return_without_mut_self() {
    // A mutable field read borrows the context exclusively through `get_field_mut`,
    // so a `&mut` return is only valid under a `&mut self` receiver and is rejected
    // under a plain `&self` receiver.
    assert_macro_rejects("cgp_auto_getter with `&mut str` under `&self`", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    fn name(&self) -> &mut str;
                }
            ),
        )
    });
}

#[test]
fn rejects_optional_slice_return() {
    // `Option<&T>` reads an `Option<T>` field, which an unsized slice cannot fill, so
    // `Option<&[u8]>` is rejected with the sized container to read instead.
    assert_macro_rejects_with(
        "cgp_auto_getter returning `Option<&[u8]>`",
        "`Option<&[T]>` would read an `Option<[T]>` field, which cannot exist because a slice is unsized; read an `Option<Vec<T>>` field as `Option<&Vec<T>>` instead",
        || {
            cgp_macro_lib::cgp_auto_getter(
                quote!(),
                quote!(
                    pub trait HasItems {
                        fn items(&self) -> Option<&[u8]>;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_optional_mutable_slice_return() {
    // The mutable form has the same unsized field, so it is rejected the same way,
    // naming the mutable read.
    assert_macro_rejects_with(
        "cgp_auto_getter returning `Option<&mut [u8]>`",
        "`Option<&mut [T]>` would read an `Option<[T]>` field, which cannot exist because a slice is unsized; read an `Option<Vec<T>>` field as `Option<&mut Vec<T>>` instead",
        || {
            cgp_macro_lib::cgp_auto_getter(
                quote!(),
                quote!(
                    pub trait HasItems {
                        fn items(&mut self) -> Option<&mut [u8]>;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_optional_trait_object_return() {
    // A trait object is unsized like a slice, so `Option<&dyn Debug>` has no field to
    // read either; the parenthesized form is the same type.
    for return_type in [
        quote!(Option<&dyn core::fmt::Debug>),
        quote!(Option<&(dyn core::fmt::Debug + Send)>),
    ] {
        assert_macro_rejects_with(
            "cgp_auto_getter returning an optional trait-object reference",
            "`Option<&dyn Trait>` would read an `Option<dyn Trait>` field, which cannot exist because a trait object is unsized; read an `Option<Box<dyn Trait>>` field as `Option<&Box<dyn Trait>>` instead",
            || {
                cgp_macro_lib::cgp_auto_getter(
                    quote!(),
                    quote!(
                        pub trait HasLabel {
                            fn label(&self) -> #return_type;
                        }
                    ),
                )
            },
        );
    }
}

#[test]
fn rejects_multiple_associated_types() {
    // A getter trait may declare at most one associated return type.
    assert_macro_rejects("cgp_auto_getter with two associated types", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    type Name;
                    type Other;
                    fn name(&self) -> &Self::Name;
                }
            ),
        )
    });
}

#[test]
fn rejects_associated_type_with_multiple_methods() {
    // An associated return type is inferred from a single field, so a trait that
    // declares one must contain exactly one getter method.
    assert_macro_rejects(
        "cgp_auto_getter with an associated type and two methods",
        || {
            cgp_macro_lib::cgp_auto_getter(
                quote!(),
                quote!(
                    pub trait HasName {
                        type Name;
                        fn name(&self) -> &Self::Name;
                        fn other(&self) -> &str;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_associated_const_trait_item() {
    // A getter trait may contain only getter methods and at most one associated
    // type; any other item (here an associated const) is rejected.
    assert_macro_rejects("cgp_auto_getter with an associated const", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {
                    const LIMIT: usize;
                    fn name(&self) -> &str;
                }
            ),
        )
    });
}

#[test]
fn rejects_attribute_argument() {
    // `#[cgp_auto_getter]` has no provider name, component, or keys to configure, so
    // any attribute argument is rejected up front.
    assert_macro_rejects("cgp_auto_getter with an attribute argument", || {
        cgp_macro_lib::cgp_auto_getter(
            quote!(GetName),
            quote!(
                pub trait HasName {
                    fn name(&self) -> &str;
                }
            ),
        )
    });
}
