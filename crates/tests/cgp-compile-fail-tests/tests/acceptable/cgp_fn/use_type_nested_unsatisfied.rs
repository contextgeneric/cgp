//! Acceptable failure: a *nested* foreign `#[use_type(HasTypes.Types, HasScalarType.Scalar in Types)]`
//! import adds the two-hop bound `<Self as HasTypes>::Types: HasScalarType` to the
//! generated trait, so a context whose `Types` associated type does not implement
//! `HasScalarType` is rejected — proof the transitively-grounded foreign bound is
//! enforced at depth, not just for a directly-named parameter.
//!
//! Before the foreign bound was carried onto the trait, this nested constraint was
//! silently dropped. CGP is now working as designed: it emits the grounded bound
//! and defers the check to `rustc`, which reports the missing `NoScalar: HasScalarType`
//! at the site that requires `App: GetScalar`.
//!
//! See docs/reference/attributes/use_type.md and docs/errors/checks/check-trait-failure.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasTypes {
    type Types;
}

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

// The generated `GetScalar` trait becomes, after the rewrite:
//   pub trait GetScalar: HasTypes
//   where <Self as HasTypes>::Types: HasScalarType
//   { fn get_scalar(&self) -> <<Self as HasTypes>::Types as HasScalarType>::Scalar; }
#[cgp_fn]
#[use_type(HasTypes.Types, HasScalarType.Scalar in Types)]
pub fn get_scalar(&self) -> Scalar {
    todo!()
}

// `App::Types` is `NoScalar`, which deliberately does not implement `HasScalarType`.
pub struct NoScalar;

pub struct App;

impl HasTypes for App {
    type Types = NoScalar;
}

// Requires `App: GetScalar`, which the blanket impl reduces to
// `<App as HasTypes>::Types: HasScalarType`, i.e. `NoScalar: HasScalarType`.
fn assert_app()
where
    App: GetScalar,
{
}

fn main() {}
