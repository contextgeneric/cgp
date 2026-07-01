//! Upstream crate for cross-crate CGP coherence tests.
//!
//! This crate defines a getter capability, a component with a provider, and a
//! component registered under the `@app` namespace. The downstream crate
//! `cgp-test-crate-b` wires these onto its own contexts, supplies its own
//! provider for the foreign `Greeter` component, and participates in the `@app`
//! namespace — exercising that CGP's two-trait split stays within Rust's
//! coherence and orphan rules across crate boundaries.
//!
//! See crates/tests/CLAUDE.md and docs/concepts/coherence.md.

use cgp::prelude::*;

/// A published field accessor. Any context with a `name` field gains it through
/// the blanket `#[cgp_auto_getter]` impl, with no wiring required.
#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

/// A component whose provider a downstream context can wire — or replace with its
/// own provider.
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

/// A component registered under the `@app` namespace. A downstream context wires
/// it through `delegate_components! { … namespace DefaultNamespace; @app.…: … }`.
#[cgp_component(Announcer)]
#[prefix(@app in DefaultNamespace)]
pub trait CanAnnounce {
    fn announce(&self) -> String;
}

#[cgp_impl(new AnnounceLoudly)]
impl Announcer
where
    Self: HasName,
{
    fn announce(&self) -> String {
        format!("ANNOUNCEMENT from {}!", self.name())
    }
}
