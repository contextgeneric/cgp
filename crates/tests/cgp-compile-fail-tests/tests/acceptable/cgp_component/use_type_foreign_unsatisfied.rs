//! Acceptable failure: a foreign `#[use_type(HasScalarType.Scalar in Types)]` import
//! adds `Types: HasScalarType` to the generated trait, so naming the component for
//! a `Types` that does not implement `HasScalarType` is rejected by the compiler.
//!
//! This is the constraint that used to be *silently dropped* — before the trait
//! carried the foreign bound, `NoScalar` would have slipped through here and only
//! failed much later (or not at all, if the abstract type went unused). CGP is now
//! working as designed: it emits the bound and defers the actual check to `rustc`,
//! which reports the missing `NoScalar: HasScalarType` at the use site.
//!
//! See docs/reference/attributes/use_type.md and docs/errors/checks/check-trait-failure.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_component(AreaCalculator)]
#[use_type(HasScalarType.Scalar in Types)]
pub trait CanCalculateArea<Types> {
    fn area(&self) -> Scalar;
}

// `NoScalar` deliberately does not implement `HasScalarType`.
pub struct NoScalar;

// Asserting the component for `NoScalar` requires `NoScalar: HasScalarType`, which
// the foreign import now demands — so this fails to compile.
pub trait CheckMissingScalar: CanCalculateArea<NoScalar> {}

fn main() {}
