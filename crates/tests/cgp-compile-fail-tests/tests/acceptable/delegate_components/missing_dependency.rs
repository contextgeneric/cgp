//! Acceptable failure: `GreetHello` carries the impl-side dependency
//! `Self: HasName`, but `Person` has no `name` field, so it does not implement
//! `HasName`. CGP wiring is lazy by design — `delegate_components!` accepts the
//! entry without checking the provider's transitive dependencies — so the unmet
//! dependency surfaces only when the consumer trait is finally called, reported
//! by the compiler through `IsProviderFor`. Using `check_components!` (or
//! `delegate_and_check_components!`) would move this same error to the wiring
//! site; deferring it to the use site is the intended lazy behavior, not a
//! macro defect.
//!
//! See docs/errors/hidden/unsatisfied-dependency.md; its surfaced counterpart is
//! docs/errors/checks/check-trait-failure.md.

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

// Accepted even though `Person` cannot satisfy `GreetHello`'s `Self: HasName`.
delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    // The unmet dependency is reported here, at the call site.
    Person { age: 0 }.greet();
}
