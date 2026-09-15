//! An `#[implicit]` argument typed as `MRef<'_, T>`: the owned-or-borrowed form.
//!
//! `#[implicit]` picks its access mode from the argument's type, and `MRef<'a, T>`
//! is the one reference-shaped mode with no mutable mirror. It requires a plain
//! `T` field and wraps the borrow as `MRef::Ref(..)`, so the body receives a value
//! that may be either owned or borrowed without the context having to commit to
//! one — and, unlike a `&mut` argument, its access never depends on the receiver,
//! so it reads through `HasField` even under `&mut self`.
//!
//! The second function pins that receiver independence. The shape-directed parse
//! — `MRef` is recognized only as a single-segment path carrying exactly a
//! lifetime and a type — is not pinned here, because every way of writing it
//! differently (nested inside another type, or reached through a qualified path)
//! falls into the owned-and-cloned mode, and `MRef` is not `Clone`, so such a case
//! cannot compile at all.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/implicit.md and
//! cgp-knowledge-base/cgp/reference/types/mref.md.

use cgp::prelude::*;

#[cgp_fn]
pub fn borrowed_name(&self, #[implicit] name: MRef<'_, String>) -> String {
    name.as_ref().to_uppercase()
}

#[cgp_fn]
pub fn borrowed_name_mut_self(&mut self, #[implicit] name: MRef<'_, String>) -> usize {
    // A `&mut self` receiver does not make an `MRef` argument a mutable read, so
    // this still resolves through `HasField` rather than `HasFieldMut`.
    name.as_ref().len()
}

#[derive(HasField)]
pub struct App {
    pub name: String,
}

#[test]
fn test_mref_implicit_argument() {
    let mut app = App {
        name: "world".to_owned(),
    };

    assert_eq!(app.borrowed_name(), "WORLD");
    assert_eq!(app.borrowed_name_mut_self(), 5);
}
