//! Problematic failure: `#[cgp_fn]` mishandles an `Option<&mut T>` implicit. The
//! implicit's outer type is a path (`Option<…>`), so the macro reads it through
//! the immutable option mode and binds an `Option<&T>` value, which then fails to
//! coerce to the `Option<&mut u8>` the signature declares. The failure lands
//! inside the generated impl body, because the macro accepts the shape rather
//! than rejecting it — only the immutable `Option<&T>` form is actually
//! supported.
//!
//! The correct behavior would be to support the mutable-option mode or to reject
//! an `Option<&mut T>` implicit at macro time. This fixture pins the current
//! behavior; its `.stderr` should improve when the defect is fixed.
//!
//! See docs/implementation/entrypoints/cgp_fn.md (Known issues).

use cgp::prelude::*;

#[cgp_fn]
fn take_slot(&self, #[implicit] slot: Option<&mut u8>) -> Option<&mut u8> {
    slot
}

fn main() {}
