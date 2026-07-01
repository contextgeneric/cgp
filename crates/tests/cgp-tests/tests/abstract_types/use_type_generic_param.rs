//! `#[use_type]` when the imported alias collides with a *generic parameter name*
//! of the trait being implemented.
//!
//! The component `Foo<T>` is implemented for the concrete parameter
//! `FooProvider<Error>`, where `Error` is the trait's generic parameter — but the
//! same name `Error` is also imported via `#[use_type(HasErrorType::Error)]`. This
//! test pins that the abstract-type import is desugared correctly into
//! `Context::Error` (the imported associated type) rather than shadowing the
//! generic parameter — i.e. `Self::Error` resolution stays distinct from the
//! `Error` type argument.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

// The `Error` parameter in `FooProvider<Error>` must desugar into `Context::Error`
// (the abstract error type imported by `#[use_type]`), not `Self::Error`.
#[cgp_impl(new FooError)]
#[use_type(HasErrorType::Error)]
impl FooProvider<Error> {
    fn foo(&self, _value: &Error) {}
}
