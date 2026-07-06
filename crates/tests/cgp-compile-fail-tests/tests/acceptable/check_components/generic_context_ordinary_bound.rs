//! Acceptable failure: the ordinary-trait-bound dependency reached through *impl
//! generics* in `delegate_components!`, rather than a concrete context.
//!
//! A generic context `<T> Wrapper<T>` wires its abstract `Scalar` type to the impl
//! generic `T` (`ScalarTypeProviderComponent: UseType<T>`), and `CompareScalars`
//! needs `Scalar: Eq` — i.e. `T: Eq`. The generic wiring is accepted unconditionally;
//! the bound only bites at a concrete instantiation. Checking `Wrapper<f64>` surfaces
//! `f64: Eq` unsatisfied through `IsProviderFor<ScalarEqualityComponent, Wrapper<f64>>`,
//! exactly as the concrete-context case does — showing the ordinary-trait-bound class
//! arises anywhere impl generics carry a bound, including a generic
//! `delegate_components!` table checked at one instantiation.
//!
//! See docs/errors/checks/ordinary-trait-bound.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_component(ScalarEquality)]
#[use_type(HasScalarType.Scalar)]
pub trait CanCompareScalars {
    fn scalars_equal(&self, a: &Scalar, b: &Scalar) -> bool;
}

#[cgp_impl(new CompareScalars)]
#[use_type(HasScalarType.Scalar)]
impl ScalarEquality
where
    Scalar: Eq,
{
    fn scalars_equal(&self, a: &Scalar, b: &Scalar) -> bool {
        a == b
    }
}

pub struct Wrapper<T>(pub T);

delegate_components! {
    <T> Wrapper<T> {
        ScalarTypeProviderComponent: UseType<T>,
        ScalarEqualityComponent: CompareScalars,
    }
}

check_components! {
    Wrapper<f64> {
        ScalarEqualityComponent,
    }
}

fn main() {}
