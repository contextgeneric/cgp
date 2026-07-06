//! Acceptable failure: two nested `#[use_type]` imports whose `in Context` clauses
//! reference *each other*, so there is no valid order in which to ground them.
//!
//! `HasA.A in B` resolves `A` against `B`, and `HasB.B in A` resolves `B` against
//! `A` — a cycle. Grounding runs to a fixpoint and deliberately stops making
//! progress on a cycle rather than looping, so the context aliases are never
//! resolved and the rewrite leaves the bare `A` and `B` from the `in` clauses in
//! type position. CGP lowers the input faithfully and defers to the compiler,
//! which reports `E0425` "cannot find type" with the caret on the unresolved
//! context alias the user wrote in the attribute.
//!
//! An *acyclic* chain in any order (`HasC.C in B, HasB.B in A, HasA.A` written
//! back-to-front) grounds fine — see the passing `use_type_fn_reverse_order`
//! behavioral test. Only a genuine cycle, which has no valid ordering, fails.
//!
//! See docs/reference/attributes/use_type.md and
//! docs/errors/lowering/unresolved-imported-type.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasA {
    type A;
}

#[cgp_type]
pub trait HasB {
    type B;
}

#[cgp_fn]
#[use_type(HasA.A in B, HasB.B in A)]
pub fn deep(&self) -> A {
    todo!()
}

fn main() {}
