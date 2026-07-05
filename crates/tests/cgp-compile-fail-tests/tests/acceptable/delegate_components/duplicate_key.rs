//! Acceptable failure: two `delegate_components!` blocks that map the same
//! component key produce two conflicting `DelegateComponent<GreeterComponent>`
//! impls for `Person`, which the Rust compiler rejects with the coherence error
//! E0119. `delegate_components!` cannot catch this — it lowers each block
//! independently and has no view of the other block — so it correctly defers to
//! the compiler, exactly as two hand-written overlapping impls would.
//!
//! See docs/errors/wiring/conflicting-wiring.md.

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
    }
}

// Re-delegating the same key in a second block emits a conflicting impl.
delegate_components! {
    Person {
        GreeterComponent: GreetGoodbye,
    }
}

fn main() {}
