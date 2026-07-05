//! Acceptable failure: two `#[cgp_impl(new GreetHello)]` blocks each declare a
//! `pub struct GreetHello;`, so the name is defined twice (E0428) and the two
//! provider impls also conflict (E0119). `#[cgp_impl]` lowers each block
//! independently and has no view of the other, so it correctly defers both to
//! the compiler, exactly as two hand-written definitions would.
//!
//! The E0428 carets fall on the `GreetHello` name inside `#[cgp_impl(new …)]`
//! rather than on the whole attribute, because the synthesized provider struct is
//! emitted with the struct ident's span (see
//! cgp-macro-core/src/types/empty_struct.rs). A regression that stamped the
//! struct with `call_site` would move the carets back onto the macro attribute.
//! The E0119 carets fall on each provider `impl` block, since those impls are the
//! user's own `#[cgp_impl]` blocks rewritten in place.
//!
//! See docs/errors/wiring/conflicting-wiring.md.

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) {}
}

// Re-declaring the same provider name emits a conflicting struct + impls.
#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) {}
}

fn main() {}
