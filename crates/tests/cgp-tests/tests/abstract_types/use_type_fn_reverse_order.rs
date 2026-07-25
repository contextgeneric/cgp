//! `#[use_type]` grounds a chain of nested foreign imports regardless of the order
//! the specs are written: `#[use_type(HasC.C in B, HasB.B in A, HasA.A)]` writes
//! the three-hop chain back-to-front, with each `in Context` referencing an alias
//! declared *after* it.
//!
//! This is the order-independence counterpart to `use_type_fn_deep_foreign`, which
//! writes the same chain front-to-back. Grounding iterates to a fixpoint over all
//! specs at once, so a spec may name a context imported by any other spec no matter
//! where it sits in the list; the bare `C` still rewrites to the three-hop
//! `<<<Self as HasA>::A as HasB>::B as HasC>::C`. Only a genuine *cycle* — which has
//! no valid order at all — fails to ground; that acceptable failure is pinned in
//! the `use_type_cyclic_context` compile-fail fixture.
//!
//! `deep` takes and returns a value of the deep type, so the test asserts a
//! concrete value flows through the fully-grounded signature at runtime.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/use_type.md and
//! cgp-knowledge-base/cgp/concepts/abstract-types.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasA {
    type A;
}

#[cgp_type]
pub trait HasB {
    type B;
}

#[cgp_type]
pub trait HasC {
    type C;
}

// The imports are written in reverse dependency order: `C in B` before `B in A`
// before the `A` that both ultimately resolve through.
#[cgp_fn]
#[use_type(HasC.C in B, HasB.B in A, HasA.A)]
pub fn deep(&self, value: C) -> C {
    value
}

pub struct Cc;

impl HasC for Cc {
    type C = u32;
}

pub struct Bb;

impl HasB for Bb {
    type B = Cc;
}

pub struct App;

impl HasA for App {
    type A = Bb;
}

#[test]
fn test_reverse_order_grounds() {
    // `<<<App as HasA>::A as HasB>::B as HasC>::C` grounds to `u32`, so `deep`
    // takes and returns a `u32` through the fully-grounded signature.
    assert_eq!(App.deep(42), 42);
}
