//! Acceptable failure: `GreetHello` carries the impl-side dependency
//! `Self: HasName`, but `Person` has no `name` field, so it cannot satisfy it.
//! `check_components!` exists precisely to surface this at the wiring site rather
//! than lazily at the call site (contrast
//! acceptable/delegate_components/missing_dependency.rs, which leaves the same
//! wiring unchecked and hits the error only when `greet` is called). The failure
//! is the check doing its job, not a macro defect.
//!
//! This fixture pins the `check_components!` error span. The unsatisfied-bound
//! caret falls on `GreeterComponent` inside the `check_components!` block, not on
//! the `Person` context type, because the check impl re-spans the shared context
//! token onto each listed component in turn with `override_span` (see
//! cgp-macro-core/src/types/check_components/table.rs). A regression that dropped
//! that re-span would report the error on the single `Person` token shared by
//! every checked component instead of on the component that actually fails.
//!
//! See docs/errors/checks/check-trait-failure.md; span mechanics: check_components.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) {
        let _ = self.name();
    }
}

#[derive(HasField)]
pub struct Person {
    pub age: u8,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

// `Person` cannot satisfy `GreetHello`'s `Self: HasName`, so the check fails here.
check_components! {
    Person {
        GreeterComponent,
    }
}

fn main() {}
