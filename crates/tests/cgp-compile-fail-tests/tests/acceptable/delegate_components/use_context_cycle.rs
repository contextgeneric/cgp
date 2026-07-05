//! Acceptable failure: wiring a component to `UseContext` when the context's only
//! implementation of that component *is* that same delegation forms a cycle.
//! `UseContext` implements the provider trait by routing back through the context's
//! own consumer-trait impl, but that consumer impl exists only via this delegation
//! to `UseContext` — so resolving `Person: Greeter<Person>` requires resolving
//! `Person: CanGreet`, which requires `Person: Greeter<Person>` again. The trait
//! solver chases the cycle until it overflows the recursion limit (`E0275`). CGP
//! lowers the wiring faithfully and cannot see that the delegation is self-referential
//! without a whole-program view, so it defers the failure to the compiler. The fix is
//! to wire the component to a concrete provider that terminates the lookup.
//!
//! See docs/errors/wiring/wiring-cycle.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

// `Person`'s only source of `CanGreet` is this delegation to `UseContext`, which
// resolves back to `CanGreet` — a cycle.
delegate_components! {
    Person {
        GreeterComponent: UseContext,
    }
}

// Forcing the wiring through a check drives the solver into the cycle directly, so it
// overflows with `E0275` and the note chain names the cycle. (A plain method call on
// `Person` would instead surface the hidden `E0599`, since the method probe treats the
// unresolvable cycle as an unsatisfied bound — see docs/errors/hidden/.)
check_components! {
    Person {
        GreeterComponent,
    }
}

fn main() {}
