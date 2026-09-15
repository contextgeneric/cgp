//! `#[use_type]` rejects imports it cannot lower unambiguously: a type-equality
//! constraint on a `#[cgp_component]` trait, two imports that resolve to the same
//! identifier or alias — whether across specs or within one braced list, and on any
//! host macro — and an import list whose contexts resolve through one another in a
//! cycle, which has no valid grounding order.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md (Tests) for
//! these failure cases and cgp-knowledge-base/cgp/reference/attributes/use_type.md for
//! the user-facing semantics.

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
fn rejects_self_referential_context() {
    // An import whose `in Context` names its own alias asks for the context to be
    // grounded before itself — a one-node cycle.
    assert_macro_rejects("use_type import whose context is its own alias", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                #[use_type(HasAType.A in A)]
                pub fn get_a(&self) -> A {
                    todo!()
                }
            ),
        )
    });
}

#[test]
fn rejects_context_cycle_between_specs() {
    // Two imports each naming the other's alias as its context: every grounding
    // order needs one of the two resolved first, so neither can be.
    assert_macro_rejects("use_type contexts forming a two-spec cycle", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                #[use_type(HasAType.A in B, HasBType.B in A)]
                pub fn deep(&self) -> A {
                    todo!()
                }
            ),
        )
    });
}

#[test]
fn rejects_context_cycle_across_three_specs() {
    // The cycle is found however long it is, so a three-hop loop is rejected the
    // same way — a search that only compared adjacent pairs would miss this.
    assert_macro_rejects("use_type contexts forming a three-spec cycle", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                #[use_type(HasAType.A in C, HasBType.B in A, HasCType.C in B)]
                pub fn deep(&self) -> A {
                    todo!()
                }
            ),
        )
    });
}

#[test]
fn rejects_cycle_through_a_trait_argument() {
    // Grounding resolves an alias in a trait path's generic arguments as well as in
    // an `in Context` clause, so a cycle running through that position is a cycle
    // too: `HasPoolType<Pool>` needs `Pool`, which is projected against it.
    assert_macro_rejects("use_type cycle through a trait argument", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                #[use_type(HasPoolType<Pool>.Pool)]
                pub fn get_pool(&self) -> Pool {
                    todo!()
                }
            ),
        )
    });
}

#[test]
fn rejects_cycle_on_component() {
    // The cycle check runs on a component trait definition too, not only on impls
    // and functions, since grounding is the same three-step transform there.
    assert_macro_rejects("use_type context cycle on a component trait", || {
        cgp_macro_lib::cgp_component(
            quote!(FooProvider),
            quote!(
                #[use_type(HasAType.A in B, HasBType.B in A)]
                pub trait CanFoo {
                    fn foo(&self) -> A;
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
