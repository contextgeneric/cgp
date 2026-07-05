//! Acceptable failure: two entries in a *single* `delegate_components!` block
//! that map the same component key produce two conflicting
//! `DelegateComponent<GreeterComponent>` impls for `Person`, rejected with the
//! coherence error E0119 — the same failure as [duplicate_key.rs], reached with
//! one block instead of two.
//!
//! This fixture exists to pin the **error span**: each conflicting entry lowers
//! to an impl re-spanned onto its own key, so E0119 points at the two distinct
//! `GreeterComponent` tokens (the "first implementation here" note lands on the
//! first entry, the conflict caret on the second) rather than at the whole
//! block. If the per-entry re-spanning in `mapping/eval.rs` regresses, both
//! carets snap back to the macro invocation and this `.stderr` changes.
//!
//! See docs/errors/wiring/conflicting-wiring.md; error-span mechanics in docs/implementation/entrypoints/delegate_components.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) {}
}

#[cgp_impl(new GreetGoodbye)]
impl Greeter {
    fn greet(&self) {}
}

pub struct Person;

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
        GreeterComponent: GreetGoodbye,
    }
}

fn main() {}
