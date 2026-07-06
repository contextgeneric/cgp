# `#[extend_where]` — the AST stack

`#[extend_where(Bound)]` adds `where` predicates to a generated trait definition, and is the only modifier that can make an arbitrary bound part of the generated *trait* rather than only its impl. It is `#[cgp_fn]`-only, and it is a modifier attribute collected by the function host; this page covers what it parses into and what it injects, and the shared collection mechanism lives in the [attribute-modifier overview](README.md). For the user-facing syntax and expansion, read the reference document [reference/attributes/extend_where.md](../../../reference/attributes/extend_where.md).

## What it parses into

`#[extend_where]` has no dedicated AST type: it parses directly into the `extend_where` field of `FunctionAttributes` — a `Vec<WherePredicate>` — populated by `Punctuated::<WherePredicate, Comma>::parse_terminated`. Because each entry is a full [`syn::WherePredicate`](https://docs.rs/syn/latest/syn/enum.WherePredicate.html) rather than a `TypeParamBound`, it can bound *any* type, not only `Self`, and can carry associated-type-equality constraints — expressiveness that `#[uses]` and `#[extend]` (which parse `TypeParamBound`s) do not have.

## What the host injects

`preprocess` adds the predicates to *both* the generated trait's own `where` clause and the impl's `where` clause. Adding them to the trait is the point of the attribute — it is what distinguishes `#[extend_where]` from [`#[uses]`](uses.md), which adds a bound to `Self` on the impl alone. Where `#[uses]` hides a dependency behind the impl, `#[extend_where]` makes a bound a visible part of the trait interface, and its full-predicate grammar is what lets it express a bound on a type other than `Self`.

## Tests

- [abstract_types/use_type_fn_nested_foreign.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_nested_foreign.rs) exercises `#[extend_where]` alongside `#[use_type]` on a `#[cgp_fn]`, where it adds a `Scalar: Copy` bound (rewritten to the two-hop path) to the generated trait's `where` clause.

## Source

- The `extend_where` field is on `FunctionAttributes` in [function.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs).
- The host that drives it: [entrypoints/cgp_fn.md](../../entrypoints/cgp_fn.md).
