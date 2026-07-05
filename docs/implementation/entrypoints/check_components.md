# `check_components!` — implementation

`check_components!` turns each entry of a check table into a compile-time assertion that a context can use a component, by generating a check trait whose supertrait is the assertion and one empty impl per checked entry. This document covers how that works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/check_components.md](../../reference/macros/check_components.md).

## Entry point

The macro is driven by the thin `check_components` function in [cgp-macro-lib/src/check_components.rs](../../../crates/macros/cgp-macro-lib/src/check_components.rs). It parses the body into a [`CheckComponentsTables`](../asts/check_components.md) — one `CheckComponentsTable` per context block — renders each to items, and emits them:

```rust
let tables: CheckComponentsTables = parse2(body)?;
let items = tables.to_items()?;
Ok(quote! { #( #items )* })
```

All real logic lives in `cgp-macro-core`. A malformed table fails while parsing. Attribute validation also happens during parsing: an unknown table-level attribute (anything other than `#[check_trait]` or `#[check_providers]`) fails with a spanned error, as do a repeated `#[check_trait]`, a repeated `#[check_providers]`, and an empty `#[check_providers()]` (which would otherwise emit a check trait with no impls that verifies nothing).

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

**The check trait name is derived from the context type's final path segment.** `derive_check_trait_ident` parses the context type through `PathWithTypeArgs` and prepends `__Check` to the last segment's identifier, so a path-qualified context such as `some_mod::Context` yields `__CheckContext` and is accepted rather than rejected — matching `delegate_components!`, which uses the context type verbatim.

## Failure modes

A few `check_components!` inputs are accepted by the macro but rejected — or silently do nothing — downstream, each intended behavior rather than a bug. Unlike the empty `#[check_providers()]` the parser rejects outright, these are left to the compiler or to the user because catching them would require second-guessing a legitimate, if degenerate, request.

An **unsatisfied dependency** is the central intended failure, and the reason the macro exists: a checked component whose provider has an impl-side dependency the context cannot meet fails with `E0277` at the check site, naming the missing bound through `IsProviderFor` rather than lazily at the eventual call site. The unsatisfied-bound caret lands on the component inside the `check_components!` block, not on the shared context type, because the check impl re-spans that one context token onto each listed component in turn with `override_span`. This is pinned by [acceptable/check_components/missing_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/missing_dependency.rs) in `cgp-compile-fail-tests`, a regression test for that re-span; the [`delegate_components!` counterpart](delegate_components.md) leaves the same wiring unchecked to show the contrasting lazy error at the call site.

An **empty check table** (`Context { }`, or every entry checking nothing) emits the check trait with no impls, so it compiles and verifies nothing. This is not rejected because a table trimmed down to nothing during editing is indistinguishable from a deliberately empty one, and an empty table causes no harm — it simply asserts nothing.

A **duplicate check entry** — the same component and parameters listed twice, whether directly (`Context { FooComponent, FooComponent }`) or through array expansion (`[A, A]: P`) — emits two identical check impls and fails with the coherence error `E0119`, exactly as two hand-written impls would. The span override aims the conflict at the repeated component.

**Two tables for the same context with no `#[check_trait]` override** both derive the same `__Check{Context}` name and emit conflicting trait definitions, failing with `E0428`. The fix is a `#[check_trait(Name)]` on one table, which is why the override exists.

## Snapshots

Every `snapshot_check_components!` invocation across the suite is indexed here, since these snapshots belong to this entrypoint:

- [checking/check_trait.rs](../../../crates/tests/cgp-tests/tests/checking/check_trait.rs) — the standalone check form: multiple check blocks in one invocation, each renamed with `#[check_trait(...)]`, per-entry parameter lists for generic-parameter components, and an array key checked against a parameter list.
- [checking/check_generic.rs](../../../crates/tests/cgp-tests/tests/checking/check_generic.rs) — a generic context (`<'a, I>` plus `where I: Clone`) whose generics and clause are carried onto each impl, a check parameter that uses a generic (`Component: &'a I`), and a component that is itself generic (`BarGetterAtComponent<I>`).
- [checking/check_providers.rs](../../../crates/tests/cgp-tests/tests/checking/check_providers.rs) — the `#[check_providers(...)]` form: the trait supertraits `IsProviderFor` and is implemented for each listed provider rather than for the context.
- [checking/check_path_context.rs](../../../crates/tests/cgp-tests/tests/checking/check_path_context.rs) — a path-qualified context (`inner::Context`): the derived trait name (`__CheckContext`) comes from the final path segment, and the impl targets the context by its full path.

No snapshot pins the plainest single-block, single-bare-component case on its own; it is covered implicitly by the richer `check_trait` block above.

## Tests

The behavioral coverage for `check_components!` is the compile-time assertion itself:

- The files listed under Snapshots are compile-only tests, so a successful build is the passing check. Each pins both the expansion (via the snapshot) and the fact that the asserted wiring resolves.
- [parser_rejections/check_components.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/check_components.rs) — the table-level attribute rejections: an empty `#[check_providers()]`, a repeated `#[check_providers]`, a repeated `#[check_trait]`, and an unknown attribute each fail to parse.
- [cgp-compile-fail-tests acceptable/check_components/missing_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/missing_dependency.rs) pins that an unsatisfied impl-side dependency is reported with its `E0277` caret on the checked component inside the block — a regression test for the `override_span` re-span (see [Failure modes](#failure-modes)).

## Source

- Entry point: `check_components` in [cgp-macro-lib/src/check_components.rs](../../../crates/macros/cgp-macro-lib/src/check_components.rs).
- Tables, entries, keys, and values: [cgp-macro-core/src/types/check_components/](../../../crates/macros/cgp-macro-core/src/types/check_components/), documented together with the `delegate_and_check_components!` stack in [asts/check_components.md](../asts/check_components.md).
- The check trait, the `#[check_trait]`/`#[check_providers]` attributes, the `__Check{Context}` name derivation, the supertrait choice, and the span override are all in `table.rs`; the cartesian-product expansion is in `entry.rs`. The span override applies the [`override_span`](../../../crates/macros/cgp-macro-core/src/functions/override_span.rs) helper, which clobbers *every* token's span unconditionally — here that is the point, since the one shared context type is forced onto each checked component in turn. This is the opposite of what `delegate_components!` needs: its sibling `override_item_span` in the same file re-spans only an impl's boundary tokens so a wired provider's own span survives for IDE navigation.
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
- The `delegate_and_check_components!` macro reuses this stack; see its [entrypoint document](delegate_and_check_components.md).
