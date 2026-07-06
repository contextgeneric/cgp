//! Acceptable failure: a provider's impl-side dependency is an *ordinary Rust
//! trait bound* — a standard trait (`Eq`), not a CGP capability — on an abstract
//! type, and the concrete type the context wires for that abstract type does not
//! implement it.
//!
//! `CompareScalars` requires `Scalar: Eq` (rewritten by `#[use_type]` to
//! `<Self as HasScalarType>::Scalar: Eq`). `App` wires its `Scalar` type to `f64`,
//! which is `PartialEq` but not `Eq`, so the dependency is unmet. The wiring is
//! accepted lazily; forcing it through `check_components!` surfaces the failure via
//! `IsProviderFor` as `E0277` — but unlike a missing `HasField` (whose leaf sits in
//! a `help:` note under a `CanUseComponent` primary), the *primary* error names the
//! ordinary bound on the concrete type directly (`f64: Eq` is not satisfied), the
//! `help:` lists the standard types that *do* implement `Eq`, and the `IsProviderFor`
//! note points at the `Scalar: Eq` bound as "introduced here". The fix is to satisfy
//! the ordinary trait (wire an `Eq` type such as an integer, or derive/impl `Eq`),
//! not to wire a component or add a field.
//!
//! CGP lowers the bound faithfully and cannot see the wired type violates it, so it
//! defers to the compiler. This is the same lazy-wiring mechanism as a CGP-capability
//! dependency; only the *kind of leaf* (an ordinary trait) and the fix differ.
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

pub struct App;

delegate_components! {
    App {
        ScalarTypeProviderComponent: UseType<f64>,
        ScalarEqualityComponent: CompareScalars,
    }
}

check_components! {
    App {
        ScalarEqualityComponent,
    }
}

fn main() {}
