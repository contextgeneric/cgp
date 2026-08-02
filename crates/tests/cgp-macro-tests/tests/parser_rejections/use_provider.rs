//! `#[use_provider]` binds exactly one inner provider per attribute, so a
//! comma-separated list of provider-and-trait pairs does not parse.
//!
//! The attribute's own argument ends in a `+`-separated bound list, which is
//! consumed to the end of the input. A comma after the first pair therefore lands
//! where the bound parser expects a `+`, and the whole attribute is rejected —
//! which makes this attribute the exception to the one-attribute-comma-separated
//! convention `#[uses]` and `#[use_type]` follow. Binding several inner providers
//! means stacking one attribute per provider.
//!
//! Both hosts that collect the attribute are covered, since the rejection comes
//! from the shared argument parser rather than from either host.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/use_provider.md for the
//! user-facing rule and cgp-knowledge-base/cgp/implementation/asts/attributes/use_provider.md
//! for the parser.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_comma_separated_pairs_on_impl() {
    assert_macro_rejects("use_provider with a comma-separated pair list", || {
        cgp_macro_lib::cgp_impl(
            quote!(new ScaledAreaCalculator<InnerA, InnerB>),
            quote!(
                #[use_provider(InnerA: AreaCalculator, InnerB: AreaCalculator)]
                impl<InnerA, InnerB> AreaCalculator {
                    fn area(&self) -> f64 {
                        InnerA::area(self) + InnerB::area(self)
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_comma_separated_pairs_on_fn() {
    assert_macro_rejects("use_provider with a comma-separated pair list on a fn", || {
        cgp_macro_lib::cgp_fn(
            quote!(),
            quote!(
                #[use_provider(InnerA: AreaCalculator, InnerB: AreaCalculator)]
                pub fn total_area(&self) -> f64 {
                    todo!()
                }
            ),
        )
    });
}

#[test]
fn accepts_plus_separated_bounds_on_one_provider() {
    // The counterpart the rejection above is easy to confuse with: `+` continues
    // *one* provider's bound list and is accepted, so the comma is what fails
    // rather than the presence of two bounds.
    let result = cgp_macro_lib::cgp_impl(
        quote!(new BothCalculator<Inner>),
        quote!(
            #[use_provider(Inner: AreaCalculator + PerimeterCalculator)]
            impl<Inner> AreaCalculator {
                fn area(&self) -> f64 {
                    Inner::area(self)
                }
            }
        ),
    );

    assert!(
        result.is_ok(),
        "expected `+`-separated bounds on one provider to be accepted",
    );
}
