//! Problematic failure: `#[cgp_fn]` does not support a mutable-slice implicit
//! argument. With a `&mut self` receiver, a `&mut [u8]` implicit skips the slice
//! case (which only matches a shared `&[T]`, since there is no `AsMut<[T]>`
//! counterpart) and falls through to the plain-reference mode over the field
//! type `[u8]`. The macro then emits a bound on `HasFieldMut<Tag, Value = [u8]>`
//! that no context can satisfy, because `[u8]` is unsized and cannot be a field
//! type. The macro *accepts* the input rather than rejecting it, so the failure
//! lands on the emitted code at the call site.
//!
//! (Note the contrast with a `&self` receiver, which the macro *does* reject
//! cleanly with "&mut self is required for mutable field reference" — that
//! rejection is covered by an `assert_macro_rejects` case in `cgp-macro-tests`,
//! not here.)
//!
//! The correct behavior would be either to support the mutable-slice mode or to
//! reject a `&mut [T]` implicit at macro time with a clear message. This fixture
//! pins the current, worse behavior; its `.stderr` should improve when the
//! defect is fixed.
//!
//! See docs/implementation/entrypoints/cgp_fn.md (Known issues).

use cgp::prelude::*;

#[cgp_fn]
fn zero_all(&mut self, #[implicit] items: &mut [u8]) {
    for x in items.iter_mut() {
        *x = 0;
    }
}

#[derive(HasField)]
pub struct Buffer {
    pub items: Vec<u8>,
}

fn main() {
    let mut buffer = Buffer {
        items: vec![1, 2, 3],
    };
    buffer.zero_all();
}
