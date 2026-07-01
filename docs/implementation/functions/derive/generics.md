# `merge_generics`

`merge_generics` combines two `syn::Generics` into one, concatenating their parameters and unioning their `where` clauses. The CGP codegen repeatedly needs to build an impl whose generics come from more than one source — a trait's own generics plus a provider's extra parameters, say — and this helper is how those are joined into a single parameter list and predicate set.

The merge is a straightforward concatenation: the parameters of the first `Generics` come before those of the second, and the predicates of both `where` clauses are collected into one (dropped entirely when neither side has any). The angle-bracket tokens are taken from the first argument, so parameter *order* follows the caller's chosen sequence — the first argument's parameters lead. There is no de-duplication or reordering, so the caller is responsible for passing parameter lists that are already free of clashes and in a valid order (lifetimes before type parameters, as Rust requires).

## Tests

- The helper has no dedicated test; it is covered indirectly through the expansion snapshots of the macros that assemble multi-source impls.

## Source

- The function lives in [cgp-macro-core/src/functions/generics/merge_generics.rs](../../../../crates/macros/cgp-macro-core/src/functions/generics/merge_generics.rs).
