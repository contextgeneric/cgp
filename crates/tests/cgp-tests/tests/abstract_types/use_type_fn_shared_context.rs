//! `#[use_type]` where two imports name the *same* alias as their context —
//! `#[use_type(HasA.A, HasB.B in A, HasC.C in A)]` — a diamond in the grounding
//! graph rather than a chain.
//!
//! Grounding resolves each spec against every other, so a shared context grounds
//! once and both dependents pick it up: `B` and `C` are each projected against
//! `<Self as HasA>::A`. The case is worth pinning separately from the linear chain
//! because it is the shape a *cycle* check must not mistake for a cycle — two edges
//! into one node revisit that node without ever returning to a node still on the
//! search path, so the graph is acyclic and must be accepted.
//!
//! The chain cases are pinned by `use_type_fn_deep_foreign` (front-to-back) and
//! `use_type_fn_reverse_order` (back-to-front); the arrangements with no valid
//! grounding order at all are rejected at macro time and pinned in
//! `cgp-macro-tests`' `parser_rejections::use_type`.
//!
//! See cgp-knowledge-base/cgp/implementation/asts/attributes/use_type.md and
//! cgp-knowledge-base/cgp/reference/attributes/use_type.md.

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

// Both `B` and `C` are imported from the same foreign context `A`, so grounding
// resolves `A` once and both specs project against the result.
#[cgp_fn]
#[use_type(HasA.A, HasB.B in A, HasC.C in A)]
pub fn combine(&self, left: B, right: C) -> (B, C) {
    (left, right)
}

pub struct Aa;

impl HasB for Aa {
    type B = u32;
}

impl HasC for Aa {
    type C = u64;
}

pub struct App;

impl HasA for App {
    type A = Aa;
}

#[test]
fn test_shared_context_grounds() {
    // `B` grounds to `<<App as HasA>::A as HasB>::B` (a `u32`) and `C` to the `u64`
    // its sibling names, so both values flow through the shared context.
    assert_eq!(App.combine(1u32, 2u64), (1u32, 2u64));
}
