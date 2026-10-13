//! `#[cgp_component]` rejects inputs it cannot lower into a component: a
//! non-trait item, a trait carrying a const generic parameter, attribute
//! arguments with a repeated key, an unknown key, or no `provider` key, a
//! `name` whose parameters the trait does not declare, a `#[uses]`
//! attribute, which belongs on a provider (`#[extend]` adds a supertrait), and
//! a method parameter that is not a plain name, which the generated impls could
//! not forward.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_component.md (Tests) for these failure
//! cases, and cgp-knowledge-base/cgp/reference/macros/cgp_component.md for the user-facing
//! semantics.

use quote::quote;

use super::{assert_macro_rejects, assert_macro_rejects_with};

#[test]
fn rejects_non_trait_item() {
    // A struct is not a trait, so the consumer-trait parser rejects it.
    assert_macro_rejects("cgp_component on a struct", || {
        cgp_macro_lib::cgp_component(
            quote!(FooProvider),
            quote!(
                pub struct NotATrait;
            ),
        )
    });
}

#[test]
fn rejects_const_generic_parameter() {
    // A const value has no place in the `IsProviderFor` params tuple (a tuple of
    // types) and cannot key CGP's type-based wiring, so a const generic parameter
    // on the component is rejected rather than lowered into non-compiling code.
    assert_macro_rejects("cgp_component with a const generic parameter", || {
        cgp_macro_lib::cgp_component(
            quote!(Foo),
            quote!(
                pub trait CanFoo<const N: usize> {
                    fn foo(&self) -> usize;
                }
            ),
        )
    });
}

#[test]
fn rejects_component_name_parameter_the_trait_lacks() {
    // The marker struct is declared with the name's parameters while the generated
    // impls name it with the trait's, so a parameter only the name carries would
    // fail downstream (`E0425`); the macro rejects it with a spanned error instead.
    assert_macro_rejects("cgp_component with an undeclared name parameter", || {
        cgp_macro_lib::cgp_component(
            quote!(provider: AreaCalculator, name: AreaCalculatorComponent<T>),
            quote!(
                pub trait CanCalculateArea {
                    fn area(&self) -> f64;
                }
            ),
        )
    });
}

#[test]
fn accepts_component_name_parameter_the_trait_declares() {
    // The same `name` form with a parameter the trait does declare expands.
    assert!(
        cgp_macro_lib::cgp_component(
            quote!(provider: AreaCalculator, name: AreaCalculatorComponent<Shape>),
            quote!(
                pub trait CanCalculateArea<Shape> {
                    fn area(&self, shape: &Shape) -> f64;
                }
            ),
        )
        .is_ok()
    );
}

#[test]
fn rejects_duplicate_key() {
    // Each of `provider`, `context`, and `name` may appear at most once.
    assert_macro_rejects("cgp_component with a repeated key", || {
        cgp_macro_lib::cgp_component(
            quote!(provider: FooProvider, provider: BarProvider),
            quote!(
                pub trait CanFoo {
                    fn foo(&self);
                }
            ),
        )
    });
}

#[test]
fn rejects_unknown_key() {
    assert_macro_rejects("cgp_component with an unknown key", || {
        cgp_macro_lib::cgp_component(
            quote!(provider: FooProvider, component: FooComponent),
            quote!(
                pub trait CanFoo {
                    fn foo(&self);
                }
            ),
        )
    });
}

#[test]
fn rejects_missing_provider_key() {
    // The keyed form has no default for `provider`.
    assert_macro_rejects("cgp_component without a provider key", || {
        cgp_macro_lib::cgp_component(
            quote!(context: Ctx),
            quote!(
                pub trait CanFoo {
                    fn foo(&self);
                }
            ),
        )
    });
}

/// `#[uses]` declares a provider's dependencies, so on a component trait the
/// macro rejects it and names the `#[extend]` form, rather than leaving an
/// attribute the compiler does not know (with its `#[used]` suggestion).
#[test]
fn rejects_uses_attribute() {
    assert_macro_rejects_with(
        "cgp_component with a #[uses] attribute",
        "`#[uses]` declares the dependencies of a provider, not of a trait; write `#[extend(HasName)]` to make it a supertrait",
        || {
            cgp_macro_lib::cgp_component(
                quote!(Greeter),
                quote!(
                    #[uses(HasName)]
                    pub trait CanGreet {
                        fn greet(&self);
                    }
                ),
            )
        },
    );
}

const UNNAMED_PARAMETER: &str = "a component method parameter needs a plain name, since the generated impls forward each argument by name; write a name such as `_value`, and destructure inside a default body";

/// The generated impls forward each argument to the provider by name, so a `_`
/// parameter, which Rust allows in a method without a body, is rejected with a
/// message saying what to write rather than an internal parse error.
#[test]
fn rejects_unnamed_parameter() {
    assert_macro_rejects_with(
        "cgp_component with a `_` parameter",
        UNNAMED_PARAMETER,
        || {
            cgp_macro_lib::cgp_component(
                quote!(Counter),
                quote!(
                    pub trait CanCount {
                        fn count(&self, _: u32) -> u32;
                    }
                ),
            )
        },
    );
}

/// A destructuring pattern in a default method has no single name to forward
/// either.
#[test]
fn rejects_destructuring_parameter() {
    assert_macro_rejects_with(
        "cgp_component with a destructuring parameter",
        UNNAMED_PARAMETER,
        || {
            cgp_macro_lib::cgp_component(
                quote!(Adder),
                quote!(
                    pub trait CanAdd {
                        fn add(&self, (a, b): (u32, u32)) -> u32 {
                            a + b
                        }
                    }
                ),
            )
        },
    );
}
