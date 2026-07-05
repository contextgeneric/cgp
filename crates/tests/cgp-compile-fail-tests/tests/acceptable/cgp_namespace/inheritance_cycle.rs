//! Acceptable failure: two namespaces that inherit from each other, so resolving
//! any key through either one chases the other and back with no terminating step —
//! a trait-resolution cycle the compiler reports as `E0275` overflow.
//!
//! `new NamespaceA: NamespaceB` emits the inheritance blanket impl
//! `impl<Table, Key, Value> NamespaceA<Table> for Key where Key: NamespaceB<..>, ..`,
//! and `new NamespaceB: NamespaceA` emits the mirror impl. Evaluating the `where`
//! clause of either impl requires evaluating the other's, which requires the first
//! again — an infinite chain. Unlike the lazy `UseContext` wiring cycle (which is
//! accepted and only overflows when forced through a check), this cycle is caught
//! **eagerly at the two `cgp_namespace!` definitions**: the compiler evaluates each
//! inheritance impl's own `where` bound and overflows, so both definitions carry an
//! `E0275`, with no joining context required. A self-inheriting `new A: A` fails the
//! same way. CGP cannot see that the parent chain is circular from one macro
//! invocation, so it lowers each namespace faithfully and defers to the compiler.
//!
//! See docs/errors/wiring/namespace-inheritance-cycle.md.

use cgp::prelude::*;

cgp_namespace! {
    new NamespaceA: NamespaceB {}
}

cgp_namespace! {
    new NamespaceB: NamespaceA {}
}

fn main() {}
