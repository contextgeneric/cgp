//! Acceptable failure: a `cgp_namespace!` block *without* `new` re-opens a foreign
//! namespace to add an entry keyed on a foreign component — an orphan violation.
//!
//! Omitting `new` tells the macro the namespace trait is declared elsewhere and to
//! emit only the entry impls. Here `AppNamespace` and `GreeterComponent` both come
//! from `cgp-test-crate-a`, so the `GreeterComponent => @foo` entry expands to
//! `impl<__Table__> AppNamespace<__Table__> for GreeterComponent { type Delegate =
//! RedirectLookup<..> }`, whose trait and self type are both foreign. No local type
//! covers the table parameter, so `__Table__` is uncovered and the orphan rule
//! rejects it (`E0210`, caret on the whole `cgp_namespace!` block, offending
//! parameter `__Table__` — the namespace table parameter, distinct from the
//! `__Components__` of a `#[default_impl]`). A crate may only add entries to a
//! namespace whose trait it owns; to extend a foreign namespace, define a *new*
//! local namespace that *inherits* it (`new Local: AppNamespace { .. }`), which is
//! orphan-safe because the emitted impls are for the local trait. CGP lowers the
//! entry faithfully; only the whole program reveals the impl is foreign, so it
//! defers to the compiler.
//!
//! This is the `cgp_namespace!` trigger of the orphan class, alongside the
//! `#[default_impl]` triggers in default_impl_foreign_prefix_path.rs and
//! default_impl_foreign_component.rs.
//!
//! See docs/errors/wiring/orphan-rule.md.

use cgp::prelude::*;
use cgp_test_crate_a::{AppNamespace, GreeterComponent};

cgp_namespace! {
    AppNamespace {
        GreeterComponent => @foo,
    }
}

fn main() {}
