//! `#[uses(...)]` on a `#[cgp_impl]` provider accepts an associated-type-equality
//! bound the same way the `#[cgp_fn]` form does. Here `ValidateWithError` imports
//! `#[uses(HasErrorType<Error = AppError>)]`, which becomes the impl-side
//! dependency `Self: HasErrorType<Error = AppError>` — pinning the context's
//! abstract error type to a concrete one. The `delegate_and_check_components!`
//! check is what makes the pin load-bearing: it holds only because `App`
//! implements `HasErrorType` with exactly `Error = AppError`; a different error
//! type would fail the check.
//!
//! The `#[uses]` equality bound is the feature under test and is written plainly.
//! The component, the direct `HasErrorType` impl, and the wiring are incidental
//! scaffolding (their expansions are owned by the `basic_delegation`,
//! `abstract_types`, and `checking` concepts), so they are written as plain
//! macros. For pinning an abstract type, `#[use_type(HasErrorType.{Error = AppError})]`
//! is the preferred spelling; this test exercises the more general bound `#[uses]`
//! accepts.
//!
//! See docs/implementation/asts/attributes.md and
//! docs/reference/attributes/uses.md.

use cgp::prelude::*;

#[derive(Debug)]
pub struct AppError(pub String);

#[cgp_component(Validator)]
pub trait CanValidate {
    fn validate(&self) -> bool;
}

#[cgp_impl(new ValidateWithError)]
#[uses(HasErrorType<Error = AppError>)]
impl Validator {
    fn validate(&self) -> bool {
        true
    }
}

pub struct App;

impl HasErrorType for App {
    type Error = AppError;
}

delegate_and_check_components! {
    App {
        ValidatorComponent: ValidateWithError,
    }
}

#[test]
fn test_impl_uses_associated_type() {
    assert!(App.validate());
}
