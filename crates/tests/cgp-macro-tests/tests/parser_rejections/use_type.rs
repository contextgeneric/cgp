//! `#[use_type]` rejects imports it cannot lower unambiguously: a type-equality
//! constraint on a `#[cgp_component]` trait, and two imports that resolve to the
//! same identifier or alias — whether across specs or within one braced list, and
//! on any host macro.
//!
//! See docs/implementation/asts/attributes.md (Tests) for these failure cases and
//! docs/reference/attributes/use_type.md for the user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_equality_on_component() {
    // A component trait definition cannot pin an abstract type to a concrete one,
    // so the `= ...` equality form is refused on `#[cgp_component]`.
    assert_macro_rejects("use_type equality on a component trait", || {
        cgp_macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[use_type(HasScalarType.{Scalar = f64})]
                pub trait CanCalculateArea {
                    fn area(&self) -> Scalar;
                }
            ),
        )
    });
}

#[test]
fn rejects_duplicate_alias_across_specs() {
    // Two specs importing under the same bare name would make the substitution
    // silently pick one and drop the other, so it is rejected.
    assert_macro_rejects("use_type duplicate alias across specs", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                #[use_type(HasFooType.Foo, HasBarType.Foo)]
                pub fn do_foo(&self) -> Foo {
                    todo!()
                }
            ),
        )
    });
}

#[test]
fn rejects_duplicate_alias_on_component() {
    // The same duplicate check applies to a component trait, not only to impls
    // and functions.
    assert_macro_rejects("use_type duplicate alias on a component trait", || {
        cgp_macro_lib::cgp_component(
            quote!(FooProvider),
            quote!(
                #[use_type(HasErrorType.Error, HasOtherErrorType.Error)]
                pub trait CanFoo {
                    fn foo(&self) -> Error;
                }
            ),
        )
    });
}

#[test]
fn rejects_duplicate_alias_within_one_braced_list() {
    // Two entries of one braced list aliasing to the same name are also a
    // duplicate, even though they belong to the same trait.
    assert_macro_rejects("use_type duplicate alias within one braced list", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                #[use_type(HasFooType.{Bar as Dup, Baz as Dup})]
                pub fn do_foo(&self) -> Dup {
                    todo!()
                }
            ),
        )
    });
}
