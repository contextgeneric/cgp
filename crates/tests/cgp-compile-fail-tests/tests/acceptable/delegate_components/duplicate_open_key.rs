//! Acceptable failure: an `open` header and an explicit mapping that both wire
//! the same component. `open GreeterComponent;` emits a `DelegateComponent<
//! GreeterComponent>` redirect impl, and the following `GreeterComponent:
//! GreetHello` emits another, so they conflict with the coherence error E0119 —
//! the third duplicate-key shape named in the Failure modes doc, alongside
//! [duplicate_key.rs] (two blocks) and [duplicate_key_same_block.rs] (two plain
//! entries).
//!
//! This fixture pins the **error span** for the `open`-header entry, whose span
//! is sourced in `statement/open.rs` from the opened component (a distinct source
//! from the plain key path): the "first implementation here" note lands on the
//! `GreeterComponent` inside `open …;`, and the conflict caret on the explicit
//! mapping's key.
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

pub struct Person;

delegate_components! {
    Person {
        open GreeterComponent;

        GreeterComponent: GreetHello,
    }
}

fn main() {}
