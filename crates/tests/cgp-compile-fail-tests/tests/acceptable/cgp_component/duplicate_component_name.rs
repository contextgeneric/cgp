//! Acceptable failure: `#[cgp_component(Greeter)]` derives a marker
//! `pub struct GreeterComponent;`, and this module also declares its own
//! `GreeterComponent`, so the name is defined twice (E0428). `#[cgp_component]`
//! expands without any view of the rest of the module, so it emits the derived
//! marker faithfully and lets the compiler report the clash — exactly as two
//! hand-written definitions would.
//!
//! This fixture pins the span of the *derived* `#[cgp_component]` marker. The
//! E0428 "previous definition of the type `GreeterComponent` here" note falls on
//! the `Greeter` provider name the user wrote inside `#[cgp_component(…)]`, not on
//! the whole attribute, because the derived marker struct ident is emitted with
//! the provider identifier's own span (see
//! cgp-macro-core/src/types/cgp_component/args/component_args.rs). A regression
//! that stamped the marker with `Span::call_site()` would move that note onto the
//! whole `#[cgp_component(..)]` attribute — the leak the span fix removed so that
//! cross-crate go-to-definition on the marker resolves to the provider name alone.
//!
//! See docs/errors/wiring/conflicting-wiring.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

// Collides with the `GreeterComponent` marker derived from `Greeter` above.
pub struct GreeterComponent;

fn main() {}
