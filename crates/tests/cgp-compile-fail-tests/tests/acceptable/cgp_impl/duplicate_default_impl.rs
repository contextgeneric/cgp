//! Acceptable failure: two `#[cgp_impl]` blocks each carrying a
//! `#[default_impl(String in DefaultImpls1<..>)]` for the same key emit two
//! conflicting `DefaultImpls1<ShowImplComponent, __Components__>` impls for
//! `String`, which the Rust compiler rejects with the coherence error E0119.
//! `#[cgp_impl]` lowers each block independently and has no view of the other,
//! so it correctly defers to the compiler, exactly as two hand-written
//! overlapping impls would.
//!
//! The carets fall on the `String` key inside `#[default_impl(...)]` rather than
//! on the whole `#[cgp_impl]` attribute, because the synthesized default-impl is
//! re-spanned onto that key token (see
//! cgp-macro-core/src/types/attributes/default_impl/attribute.rs). A regression
//! that dropped the re-span would move the carets back onto the macro attribute.
//!
//! See docs/errors/wiring/conflicting-wiring.md.

use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;

#[cgp_component(ShowImpl)]
pub trait Show<T> {
    fn show(&self, value: &T) -> String;
}

#[cgp_impl(new ShowStringA)]
#[default_impl(String in DefaultImpls1<ShowImplComponent>)]
impl ShowImpl<String> {
    fn show(&self, value: &String) -> String {
        value.clone()
    }
}

// Re-registering the same key as a per-type default emits a conflicting impl.
#[cgp_impl(new ShowStringB)]
#[default_impl(String in DefaultImpls1<ShowImplComponent>)]
impl ShowImpl<String> {
    fn show(&self, value: &String) -> String {
        value.clone()
    }
}

fn main() {}
