//! Acceptable failure: a downstream crate registers a `#[default_impl]` for a
//! foreign *unprefixed* component into a foreign namespace — an orphan violation
//! even though the component key is a bare marker rather than a prefix path.
//!
//! `GreeterComponent` (unprefixed) and `AppNamespace` both come from
//! `cgp-test-crate-a`, so `#[default_impl(GreeterComponent in AppNamespace)]`
//! expands to `impl<__Components__> AppNamespace<__Components__> for GreeterComponent`,
//! whose trait (`AppNamespace`) and self type (`GreeterComponent`) are both foreign
//! to this crate. Rust accepts a foreign-trait impl only when a local type covers
//! its type parameters, and here none does, so `__Components__` is an uncovered
//! type parameter and the orphan rule rejects it (`E0210`, caret on the
//! `#[cgp_impl]` attribute that generated the impl). Registering a per-component
//! default therefore needs the crate to own *either* the namespace or the component
//! key; owning neither, this crate cannot.
//!
//! This is the bare-marker sibling of default_impl_foreign_prefix_path.rs, whose
//! key is a foreign `PathCons<..>` path: both are the same orphan violation, and
//! this one shows the restriction is not specific to prefixed components. The
//! orphan-*safe* counterpart — a *local* component key registered into the foreign
//! `AppNamespace` — is exercised in `cgp-test-crate-b`.
//!
//! See docs/errors/wiring/orphan-rule.md.

use cgp::prelude::*;
use cgp_test_crate_a::{AppNamespace, Greeter, GreeterComponent, HasName};

#[cgp_impl(new GreetPolitely)]
#[default_impl(GreeterComponent in AppNamespace)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) -> String {
        format!("Good day, {}", self.name())
    }
}

fn main() {}
