//! Acceptable failure: `#[use_type]` imports an associated type name the owning
//! trait does not declare, so the substituted `<Self as Trait>::WrongName` path
//! names an associated type that does not exist and the compiler rejects it.
//!
//! `HasScalarType` declares `Scalar`, but the import names `Scalr` (a typo), so the
//! bare `Scalr` in the signature is rewritten to `<Self as HasScalarType>::Scalr`.
//! CGP cannot know the trait's associated types at expansion time — it performs a
//! textual rewrite — so it lowers the name faithfully and defers to the compiler,
//! which reports `E0576` "cannot find associated type `Scalr`". Because the
//! substitution preserves the *span* of the identifier the user wrote, the caret
//! lands on the `Scalr` in the signature, not on the macro attribute — so this
//! fixture also guards that span behavior.
//!
//! See docs/reference/attributes/use_type.md and
//! docs/errors/lowering/unresolved-imported-type.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_fn]
#[use_type(HasScalarType.Scalr)]
pub fn get_scalar(&self) -> Scalr {
    todo!()
}

fn main() {}
