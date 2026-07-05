//! Acceptable failure: a context joins a namespace that *routes* a prefixed
//! component to a path, but no entry ever *terminates* that path with a provider,
//! so the namespace lookup finds no delegate.
//!
//! `CanGreet` carries `#[prefix(@app in DefaultNamespace)]`, so `DefaultNamespace`
//! resolves `GreeterComponent` to `RedirectLookup<_, @app.GreeterComponent>`. `App`
//! joins `DefaultNamespace` with `namespace DefaultNamespace;`, so its
//! `GreeterComponent` lookup follows that redirect — but nothing (no `#[default_impl]`,
//! no namespace body entry, no direct `@app.GreeterComponent:` line) ever binds a
//! provider at that path. The defined `GreetHello` is never wired there. Resolving
//! the component therefore requires `App: DelegateComponent<@app.GreeterComponent>`,
//! for which there is no impl, and the `check_components!` surfaces that as an
//! `E0277` on `PathCons<app, GreeterComponent>: DefaultNamespace<App>`.
//!
//! This is the *lookup-failed* class — no provider is found at all — distinct from
//! an unsatisfied *dependency*, where a provider is found but its `where` clause is
//! unmet. The forgotten binding (usually a missing `#[default_impl]` or body entry)
//! is the common namespace mistake it captures. CGP lowers the wiring faithfully;
//! only the whole program reveals the path is unbound, so it defers to the compiler.
//!
//! See docs/errors/checks/unregistered-namespace-path.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
#[prefix(@app in DefaultNamespace)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) -> String {
        "Hello".to_owned()
    }
}

pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;
    }
}

check_components! {
    App {
        GreeterComponent,
    }
}

fn main() {}
