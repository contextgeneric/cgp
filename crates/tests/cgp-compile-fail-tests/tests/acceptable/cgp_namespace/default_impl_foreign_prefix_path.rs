//! Acceptable failure: a downstream crate cannot register a default for a
//! *prefixed* upstream component into the upstream namespace with `#[default_impl]`.
//!
//! `cgp-test-crate-a`'s `Announcer` carries `#[prefix(@app in DefaultNamespace)]`,
//! so its namespace key is the path `@app.AnnouncerComponent`. Registering a
//! default at that path in crate-a's `AppNamespace` expands to
//! `impl AppNamespace<_> for PathCons<Symbol<"app">, PathCons<AnnouncerComponent, Nil>>`,
//! whose trait (`AppNamespace`) and every element of the `Self` type (`PathCons`
//! and `Symbol` from `cgp`, `AnnouncerComponent` from crate-a) are all foreign to
//! this crate — an orphan-rule violation (E0117). A per-component default keyed on
//! a *prefix path* can therefore only be written in the crate that owns the
//! namespace; a downstream crate must own the key, which for a prefixed component
//! it does not. (The orphan-safe counterpart — a *local* component key registered
//! into a foreign namespace — is exercised in `cgp-test-crate-b`.)
//!
//! This is why `#[default_impl]` couples an implementation to the namespace's
//! crate and why the guide recommends namespace *body* entries for wiring that
//! must live downstream of the namespace.
//!
//! See docs/implementation/entrypoints/cgp_namespace.md (Failure modes).

use cgp::prelude::*;
use cgp_test_crate_a::{Announcer, AnnouncerComponent, AppNamespace, HasName};

#[cgp_impl(new AnnounceQuietly)]
#[default_impl(@app.AnnouncerComponent in AppNamespace)]
impl Announcer
where
    Self: HasName,
{
    fn announce(&self) -> String {
        format!("(psst, {})", self.name())
    }
}

fn main() {}
