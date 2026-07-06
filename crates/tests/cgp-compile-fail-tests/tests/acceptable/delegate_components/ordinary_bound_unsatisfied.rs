//! Acceptable failure: the same unmet *ordinary Rust trait bound* dependency as
//! check_components/ordinary_bound_unsatisfied.rs (`Scalar: Eq` with `f64` wired),
//! but exercised by calling the consumer method rather than a check — so the cause
//! is *hidden*.
//!
//! Calling `app.scalars_equal(..)` produces the `E0599` "method exists but its
//! trait bounds were not satisfied" shape: it names `App: CanCompareScalars` /
//! `App: ScalarEquality<App>`, misclassifies the method as an associated function
//! (the provider method has no `self` receiver), and suggests `App::scalars_equal()`
//! — but never mentions the unmet `f64: Eq`. This is byte-for-shape identical to the
//! HasName hidden case in delegate_components/missing_dependency.rs: the compiler's
//! method-probe heuristic drops the nested `where`-clause bound regardless of whether
//! that bound is a `HasField`, a CGP capability, or an ordinary trait. Promote it with
//! `check_components!` to surface the `f64: Eq` cause.
//!
//! See docs/errors/hidden/unsatisfied-dependency.md; the surfaced counterpart is
//! docs/errors/checks/ordinary-trait-bound.md.

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

pub struct App;

delegate_components! {
    App {
        ScalarTypeProviderComponent: UseType<f64>,
        ScalarEqualityComponent: CompareScalars,
    }
}

fn main() {
    let app = App;
    let _ = app.scalars_equal(&1.0, &2.0);
}
