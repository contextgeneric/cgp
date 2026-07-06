//! Acceptable failure: a child namespace that inherits a parent and then
//! *redefines* a key the parent already binds — a namespace entry cannot be
//! overridden by an inheriting namespace.
//!
//! `new ChildNs: BaseNs` emits the inheritance blanket impl `impl<Table, Key,
//! Value> ChildNs<Table> for Key where Key: BaseNs<__ChildNsComponents>, Key:
//! BaseNs<Table, Delegate = Value>`, which forwards *every* key `BaseNs` resolves —
//! including `GreeterComponent`, since `BaseNs` binds it. The child's own
//! `GreeterComponent: GreetBye` entry emits a second impl `impl<Table> ChildNs<Table>
//! for GreeterComponent`, and the two overlap for that key, so coherence rejects the
//! pair (`E0119`, a *single* conflict on `ChildNs<_> for GreeterComponent`, since a
//! namespace emits only its own lookup-trait impl, not the context-side
//! `DelegateComponent`/`IsProviderFor` pair). Inheritance layers new keys onto a
//! parent; it cannot revise the parent's existing keys. To vary a key per
//! configuration, leave it *unbound* in the shared base and bind it in each child,
//! rather than binding it in the base and overriding it. CGP lowers both impls
//! faithfully; only the whole program reveals the overlap, so it defers to the
//! compiler.
//!
//! This is the namespace-level (inheritance) shape of the override-conflict class;
//! contrast the context-level shape in override_registered_path.rs, where a context
//! joining a namespace tries to override a path the namespace registers.
//!
//! See docs/errors/wiring/namespace-override-conflict.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) -> String {
        "Hello".to_owned()
    }
}

#[cgp_impl(new GreetBye)]
impl Greeter {
    fn greet(&self) -> String {
        "Bye".to_owned()
    }
}

cgp_namespace! {
    new BaseNs {
        GreeterComponent: GreetHello,
    }
}

cgp_namespace! {
    new ChildNs: BaseNs {
        GreeterComponent: GreetBye,
    }
}

fn main() {}
