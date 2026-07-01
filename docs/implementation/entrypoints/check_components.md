# `check_components!` — implementation

`check_components!` turns each entry of a check table into a compile-time assertion that a context can use a component, by generating a check trait whose supertrait is the assertion and one empty impl per checked entry. This document covers how that works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/check_components.md](../../reference/macros/check_components.md).

## Entry point

The macro is driven by the thin `check_components` function in [cgp-macro-lib/src/check_components.rs](../../../crates/macros/cgp-macro-lib/src/check_components.rs). It parses the body into a [`CheckComponentsTables`](../asts/check_components.md) — one `CheckComponentsTable` per context block — renders each to items, and emits them:

```rust
let tables: CheckComponentsTables = parse2(body)?;
let items = tables.to_items()?;
Ok(quote! { #( #items )* })
```

All real logic lives in `cgp-macro-core`. A malformed table fails while parsing, and an unknown table-level attribute (anything other than `#[check_trait]` or `#[check_providers]`) fails with a spanned error during parsing.

## Pipeline

The macro parses into the [`check_components` AST stack](../asts/check_components.md) and then calls `to_items` on each table, which internally runs a single `eval` per table. Parsing splits each table into its attributes, an optional leading generic list, the context type, an optional `where` clause, and the brace-delimited check entries. `eval` builds the check trait once and then, for each evaluated entry, emits one impl of that trait; there is no multi-stage lowering beyond this.

## Generated items

Each table emits one check trait followed by one empty impl per checked entry. The trait is an alias whose sole supertrait is the assertion being made, and each impl compiles only if that supertrait holds for the entry. A bare component with no parameters lowers to a unit `__Params__`:

```rust
// check_components! { Person { GreeterComponent } }
trait __CheckPerson<__Component__, __Params__: ?Sized>:
    CanUseComponent<__Component__, __Params__>
{}
impl __CheckPerson<GreeterComponent, ()> for Person {}
```

The impl holds only if `Person: CanUseComponent<GreeterComponent, ()>`, which routes through `IsProviderFor` so an unsatisfied transitive bound (a missing `HasField`, say) is what the compiler reports. The generic parameters are literally `__Component__` and `__Params__` in the output, and the check trait name defaults to `__Check{Context}`, derived from the context type's leading identifier.

A component with parameters places them in the `__Params__` slot — a single parameter directly, multiple as a tuple. The `#[check_providers(...)]` form changes both the supertrait and the implementing type: the trait supertraits `IsProviderFor<__Component__, Context, __Params__>` instead of `CanUseComponent`, and one impl is written for each listed provider rather than for the context, so each provider is asserted independently.

## Behavior and corner cases

**Array syntax expands to the cartesian product.** A bracketed key, a bracketed value, or both, expand to one entry per combination before any impl is emitted, so `[A, B]: [P, Q]` yields four impls. The key and value sides are parsed independently (`CheckKey`, `CheckValue`), and each evaluated entry pairs one key with one value.

**Table-level generics and `where` clauses are merged onto every impl.** A `<'a, I> Context where I: Clone { … }` table threads both onto each generated impl, and a check parameter may itself be generic (`Component: &'a I` or a value carrying its own `<T>` list), whose generics merge with the table's.

**The error span is moved onto the component.** For the context-checking form the macro overrides the span of the context type in each impl with the span of the checked component (or parameter), so an unsatisfied-constraint error is highlighted on the component the user wrote rather than on the context. The `#[check_providers(...)]` form skips this, since it implements for the providers instead.

**A component with no value** emits a single unit-params entry; a bracketed value that is empty is treated the same way.

## Snapshots

Every `snapshot_check_components!` invocation across the suite is indexed here, since these snapshots belong to this entrypoint:

- [checking/check_trait.rs](../../../crates/tests/cgp-tests/tests/checking/check_trait.rs) — the standalone check form: multiple check blocks in one invocation, each renamed with `#[check_trait(...)]`, per-entry parameter lists for generic-parameter components, and an array key checked against a parameter list.
- [checking/check_generic.rs](../../../crates/tests/cgp-tests/tests/checking/check_generic.rs) — a generic context (`<'a, I>` plus `where I: Clone`) whose generics and clause are carried onto each impl, a check parameter that uses a generic (`Component: &'a I`), and a component that is itself generic (`BarGetterAtComponent<I>`).
- [checking/check_providers.rs](../../../crates/tests/cgp-tests/tests/checking/check_providers.rs) — the `#[check_providers(...)]` form: the trait supertraits `IsProviderFor` and is implemented for each listed provider rather than for the context.

No snapshot pins the plainest single-block, single-bare-component case on its own; it is covered implicitly by the richer `check_trait` block above.

## Tests

The behavioral coverage for `check_components!` is the compile-time assertion itself:

- The files listed under Snapshots are compile-only tests, so a successful build is the passing check. Each pins both the expansion (via the snapshot) and the fact that the asserted wiring resolves.
- There are no `cgp-macro-tests` failure cases for the check family.

## Source

- Entry point: `check_components` in [cgp-macro-lib/src/check_components.rs](../../../crates/macros/cgp-macro-lib/src/check_components.rs).
- Tables, entries, keys, and values: [cgp-macro-core/src/types/check_components/](../../../crates/macros/cgp-macro-core/src/types/check_components/), documented together with the `delegate_and_check_components!` stack in [asts/check_components.md](../asts/check_components.md).
- The check trait, the `#[check_trait]`/`#[check_providers]` attributes, the `__Check{Context}` name derivation, the supertrait choice, and the span override are all in `table.rs`; the cartesian-product expansion is in `entry.rs`.
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
- The `delegate_and_check_components!` macro reuses this stack; see its [entrypoint document](delegate_and_check_components.md).
