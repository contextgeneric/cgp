//! The getter macros reject any trait whose methods break the getter-method
//! contract. `#[cgp_getter]` and `#[cgp_auto_getter]` share the `parse_getter_fields`
//! parser, so the method-shape rejections below are driven through the
//! argument-free `cgp_auto_getter` entrypoint and hold identically for
//! `#[cgp_getter]`. A getter method must be a plain (non-const, non-async,
//! non-unsafe, non-generic) method whose first argument is a reference; a mutable
//! return requires a `&mut self` receiver. At most one associated type is
//! allowed, and only beside exactly one getter method: `#[cgp_auto_getter]`
//! rejects a type with no method, while `#[cgp_getter]` on a type alone is
//! forwarded to `#[cgp_type]`. An empty trait, or any item that is neither a
//! field method nor an associated type, is rejected. `#[cgp_auto_getter]` additionally
//! rejects any attribute argument, since it has no provider name or keys to accept.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/asts/cgp_getter.md (Tests) for these failure cases and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_getter.md for the user-facing getter-method
//! rules.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_const_getter_method() {
    assert_macro_rejects("cgp_auto_getter with a const getter method", || {
        crate::macro_lib::cgp_auto_getter(
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
        crate::macro_lib::cgp_auto_getter(
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
        crate::macro_lib::cgp_auto_getter(
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
        crate::macro_lib::cgp_auto_getter(
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
        crate::macro_lib::cgp_auto_getter(
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
        crate::macro_lib::cgp_auto_getter(
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
fn rejects_multiple_associated_types() {
    // A getter trait may declare at most one associated return type.
    assert_macro_rejects("cgp_auto_getter with two associated types", || {
        crate::macro_lib::cgp_auto_getter(
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
fn rejects_associated_type_without_method() {
    // An auto getter binds the associated type to a field read, so the type
    // needs exactly one getter method. A type on its own is a `#[cgp_type]`
    // (or a `#[cgp_getter]` that takes that path).
    assert_macro_rejects(
        "cgp_auto_getter with an associated type and no method",
        || {
            crate::macro_lib::cgp_auto_getter(
                quote!(),
                quote!(
                    pub trait HasName {
                        type Name: Display;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_associated_type_with_multiple_methods() {
    // An associated return type is inferred from a single field, so a trait that
    // declares one must contain exactly one getter method.
    assert_macro_rejects(
        "cgp_auto_getter with an associated type and two methods",
        || {
            crate::macro_lib::cgp_auto_getter(
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
        crate::macro_lib::cgp_auto_getter(
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
fn rejects_empty_getter_trait() {
    // A getter trait has to contribute a field method or an associated type;
    // an empty trait would expand to a blanket impl that checks nothing.
    assert_macro_rejects("cgp_auto_getter with an empty trait", || {
        crate::macro_lib::cgp_auto_getter(
            quote!(),
            quote!(
                pub trait HasName {}
            ),
        )
    });
}

#[test]
fn rejects_attribute_argument() {
    // `#[cgp_auto_getter]` has no provider name, component, or keys to configure, so
    // any attribute argument is rejected up front.
    assert_macro_rejects("cgp_auto_getter with an attribute argument", || {
        crate::macro_lib::cgp_auto_getter(
            quote!(GetName),
            quote!(
                pub trait HasName {
                    fn name(&self) -> &str;
                }
            ),
        )
    });
}
