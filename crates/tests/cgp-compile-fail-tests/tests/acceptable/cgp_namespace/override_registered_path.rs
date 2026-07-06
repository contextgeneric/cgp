//! Acceptable failure: a context that joins a namespace with `namespace N;`
//! cannot also wire, directly on itself, a path that `N` already registers.
//!
//! `GreetHello` registers the path `@app.GreeterComponent` into `AppNamespace`
//! with `#[default_impl]`, so `PathCons<app, GreeterComponent>` implements
//! `AppNamespace<_>`. The `namespace AppNamespace;` header then emits a blanket
//! `impl<Key> DelegateComponent<Key> for App where Key: AppNamespace<App>`, which
//! already covers that path. The extra `@app.GreeterComponent: GreetBye` entry
//! emits a second `DelegateComponent<PathCons<app, GreeterComponent>> for App`,
//! and the two overlap — E0119. CGP lowers both entries faithfully; only the whole
//! program reveals the overlap, so it defers to the compiler.
//!
//! The rule this pins: override a component the namespace routes by shadowing its
//! *marker* only when the namespace does not itself terminate the redirect path,
//! or wire the override on a path the namespace never registers. A namespace that
//! registers the leaf path leaves nothing for the context to override there.
//!
//! This is the context-level (join) shape of the override-conflict class; contrast
//! the namespace-level (inheritance) shape in inherited_override_conflict.rs, where
//! a child namespace tries to override an entry its parent binds.
//!
//! See docs/errors/wiring/namespace-override-conflict.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
#[prefix(@app in DefaultNamespace)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
#[default_impl(@app.GreeterComponent in AppNamespace)]
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
    new AppNamespace: DefaultNamespace {}
}

pub struct App;

delegate_components! {
    App {
        namespace AppNamespace;

        @app.GreeterComponent: GreetBye,
    }
}

fn main() {}
