//! `#[cgp_provider]` rejects inputs it cannot derive an `IsProviderFor` impl
//! from: an inherent `impl` with no trait, a provider trait carrying no type
//! argument (so there is no context), a const argument in the provider trait's
//! argument list, a `new`-keyword struct declaration (which belongs to
//! `#[cgp_new_provider]`), and a non-`impl` item.
//!
//! `#[cgp_new_provider]` shares this stack and rejects the same inputs.
//!
//! See docs/implementation/entrypoints/cgp_provider.md (Tests) for these failure
//! cases, and docs/reference/macros/cgp_provider.md for the user-facing semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_inherent_impl() {
    // An inherent `impl Foo { … }` has no provider trait to derive the component
    // name and `IsProviderFor` impl from.
    assert_macro_rejects("cgp_provider on an inherent impl", || {
        cgp_macro_lib::cgp_provider(
            quote!(),
            quote!(
                impl RectangleArea {
                    fn area(_context: &()) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_provider_trait_without_context() {
    // The provider trait must carry a leading type argument to serve as the
    // context; a bare `AreaCalculator` (the consumer-trait shape) has none.
    assert_macro_rejects("cgp_provider with no context type argument", || {
        cgp_macro_lib::cgp_provider(
            quote!(),
            quote!(
                impl AreaCalculator for RectangleArea {
                    fn area(_context: &()) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_const_argument_in_trait_args() {
    // A const value cannot sit in the `IsProviderFor` params tuple (a tuple of
    // types), so a const argument in the provider trait's own argument list is
    // rejected. A const generic on the provider *struct* is fine.
    assert_macro_rejects("cgp_provider with a const trait argument", || {
        cgp_macro_lib::cgp_provider(
            quote!(),
            quote!(
                impl<Context> AreaCalculator<Context, 3> for RectangleArea {
                    fn area(_context: &Context) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_new_struct_declaration() {
    // `#[cgp_provider]`'s argument is a component type only; declaring the
    // provider struct is `#[cgp_new_provider]`'s job. The `new Name` form is not
    // part of `#[cgp_provider]`'s grammar, so the trailing name is an unexpected
    // token rather than a silent struct declaration.
    assert_macro_rejects("cgp_provider(new Name)", || {
        cgp_macro_lib::cgp_provider(
            quote!(new RectangleArea),
            quote!(
                impl<Context> AreaCalculator<Context> for RectangleArea {
                    fn area(_context: &Context) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_non_impl_item() {
    // `#[cgp_provider]` only applies to an `impl` block, so a struct fails at the
    // `ItemImpl` parse.
    assert_macro_rejects("cgp_provider on a struct", || {
        cgp_macro_lib::cgp_provider(
            quote!(),
            quote!(
                pub struct NotAnImpl;
            ),
        )
    });
}
