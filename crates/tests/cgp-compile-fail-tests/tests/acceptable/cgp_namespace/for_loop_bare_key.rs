//! Acceptable failure: a `for` loop that wires a **bare key** (`Key: Value`)
//! instead of embedding it in a path, in a context that also joins a namespace —
//! the two blanket impls overlap and the compiler rejects them with `E0119`.
//!
//! `namespace DefaultNamespace;` emits a blanket `impl<Key, Value>
//! DelegateComponent<Key> for App where Key: DefaultNamespace<App, ..>` (plus the
//! matching `IsProviderFor` forwarding) that covers *every* key. A `for <Key, Value>
//! in GreeterTable { Key: Value }` loop emits a second blanket `impl<Key, Value>
//! DelegateComponent<Key> for App where Key: GreeterTable<App, ..>` — also over every
//! key — and the two overlap because a key could satisfy both `where` clauses, so
//! coherence rejects the pair (`E0119`, fully generic `DelegateComponent<_>` /
//! `IsProviderFor<_, _, _>`). This is why a loop key must sit inside a path
//! (`@app.SomeComponent.Key: Value`), which keys the impl on a concrete path rather
//! than on every key. CGP lowers both blanket impls faithfully; only the whole
//! program reveals the overlap, so it defers to the compiler.
//!
//! This is the blanket-vs-blanket shape of the overlapping-forwarding class,
//! alongside two_namespaces_joined.rs (two `namespace` joins on one context);
//! contrast the specific-vs-blanket override in override_registered_path.rs.
//!
//! See docs/errors/wiring/namespace-forwarding-conflict.md.

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

cgp_namespace! {
    new GreeterTable {
        GreeterComponent: GreetHello,
    }
}

pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;

        for <Key, Value> in GreeterTable {
            Key: Value,
        }
    }
}

fn main() {}
