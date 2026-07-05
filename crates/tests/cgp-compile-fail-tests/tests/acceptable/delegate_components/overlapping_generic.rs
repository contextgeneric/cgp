//! Acceptable failure: a generic `delegate_components!` entry that wires every
//! `Wrapper<T>` overlaps a second entry that wires the specific `Wrapper<u64>`.
//! Stable Rust has no specialization, so the two `DelegateComponent` impls
//! overlap at `Wrapper<u64>` and the compiler rejects them with E0119.
//! `delegate_components!` expands each entry to the impl the user asked for and
//! defers the overlap check to the compiler, the same as two overlapping
//! hand-written generic impls.
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

pub struct Wrapper<T>(pub T);

// Wires the whole `Wrapper<T>` family.
delegate_components! {
    <T> Wrapper<T> {
        GreeterComponent: GreetHello,
    }
}

// Overlaps the generic entry at `Wrapper<u64>`.
delegate_components! {
    Wrapper<u64> {
        GreeterComponent: GreetHello,
    }
}

fn main() {}
