//! Acceptable failure: a context that joins **two** namespaces at once —
//! `namespace NamespaceA; namespace NamespaceB;` — cannot compile, because each
//! join emits a *blanket* forwarding impl over every key and the two overlap.
//!
//! Each `namespace N;` header emits `impl<Key, Value> DelegateComponent<Key> for
//! App where Key: N<App, ..>` (plus the matching `IsProviderFor` forwarding), a
//! blanket impl that covers *every* key. Joining two namespaces emits two such
//! blanket impls — one keyed through `NamespaceA`, one through `NamespaceB` — and
//! because a key could satisfy both `where` clauses, coherence cannot prove they
//! never overlap and rejects the pair (`E0119`, fully generic `DelegateComponent<_>`
//! / `IsProviderFor<_, _, _>`, carets on the two `namespace` lines, no downstream
//! note). A context therefore forwards through at most one namespace; layer several
//! by having that one namespace *inherit* the others (`new Combined: A { .. }`
//! inheriting further), not by joining several on the context. CGP lowers both
//! blanket impls faithfully; only the whole program reveals the overlap, so it
//! defers to the compiler.
//!
//! This is the blanket-vs-blanket shape of the overlapping-forwarding class,
//! alongside for_loop_bare_key.rs (a namespace join plus a bare-key `for` loop);
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
    new NamespaceA {
        GreeterComponent: GreetHello,
    }
}

cgp_namespace! {
    new NamespaceB {}
}

pub struct App;

delegate_components! {
    App {
        namespace NamespaceA;
        namespace NamespaceB;
    }
}

fn main() {}
