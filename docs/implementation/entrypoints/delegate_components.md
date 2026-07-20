# `delegate_components!` — implementation

`delegate_components!` builds a context's type-level wiring table by parsing a `DelegateTable` from the macro body and lowering each mapping into a `DelegateComponent` impl plus a forwarding `IsProviderFor` impl. This document covers how that lowering works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/delegate_components.md](../../reference/macros/delegate_components.md).

## Entry point

The macro is driven by the thin `delegate_components` function in [cgp-macro-lib/src/delegate_components.rs](../../../crates/macros/cgp-macro-lib/src/delegate_components.rs). It parses the whole body into a single [`DelegateTable`](../asts/delegate_component.md), rejects any attributes the parser accepted (the table supports none), evaluates the table, and emits the resulting tokens:

```rust
let table: DelegateTable = parse2(body.clone())?;
table.validate_attributes()?;
let evaluated_table = table.eval()?;
Ok(evaluated_table.to_token_stream())
```

All real logic lives in `cgp-macro-core`. A malformed body fails while parsing `DelegateTable`, and an attribute on the table or any key fails in `validate_attributes` with a spanned "unsupported attribute" error rather than being silently dropped. The check recurses through mapping values, so an attribute on a key nested inside a `UseDelegate<new Inner { … }>` table is rejected too, not just one on a top-level key.

## Pipeline

The macro has two stages after parsing: attribute rejection and a single `eval`. Parsing produces the whole [`delegate_component` AST stack](../asts/delegate_component.md) — the table, its `new` keyword and optional generic list, the entries (statements plus mappings), and the keys and values inside each mapping. `eval` walks that tree once, lowering every mapping and statement into a flat list of evaluated entries and rendering each into its impl pair. The `open`/`namespace`/`for` statements and the nested-`UseDelegate` values are handled inside `eval` as part of the same walk; there is no separate preprocessing stage.

## Generated items

For every table entry the macro emits two impls in order: a `DelegateComponent` impl that records the mapping (the component key, the chosen provider as the `Delegate` type) and an `IsProviderFor` impl that forwards the provider's dependencies back through the table so a missing transitive requirement stays diagnosable. A plain `Key: Provider` mapping lowers directly:

```rust
// delegate_components! { Rectangle { AreaCalculatorComponent: RectangleArea } }
impl DelegateComponent<AreaCalculatorComponent> for Rectangle {
    type Delegate = RectangleArea;
}
impl<__Context__, __Params__>
    IsProviderFor<AreaCalculatorComponent, __Context__, __Params__> for Rectangle
where
    RectangleArea: IsProviderFor<AreaCalculatorComponent, __Context__, __Params__>,
{}
```

Both `__Context__` and `__Params__` are the reserved identifiers that appear literally in the output. When the body carries a leading `new` keyword, the macro additionally emits the target struct (`struct Rectangle;`, or a generic struct if the target carries parameters), and a nested-`UseDelegate` value lifts its inner table out into its own struct and impls, so a value like `UseDelegate<new Inner { … }>` contributes both the outer entry and a full inner table.

The `open` header and `@Component.Key` entries lower through the [`RedirectLookup`](cgp_component.md) impl that every `#[cgp_component]` already generates. The header wires each opened component to a redirect rooted at the component name in the context's own table, and each `@`-path entry stores its provider under the extended path key:

```rust
// open AreaCalculatorComponent;  →  the redirect entry
impl DelegateComponent<AreaCalculatorComponent> for MyApp {
    type Delegate = RedirectLookup<MyApp, PathCons<AreaCalculatorComponent, Nil>>;
}
// @AreaCalculatorComponent.Rectangle: RectangleArea  →  a keyed entry on the same table,
// keyed by the redirect path with a trailing wildcard, mapping to RectangleArea
```

The per-value entries are ordinary `DelegateComponent`/`IsProviderFor` pairs whose key is the redirect path type; `RedirectLookup` appends the dispatch parameter onto the path at lookup time and reads the result back.

## Behavior and corner cases

A **mapping operator** selects which value lowering applies. `:` (Normal) maps the key straight to the named provider; `->` (Direct) sets the `Delegate` to `<Value as DelegateComponent<Key>>::Delegate` and adds a `Value: DelegateComponent<Key>` bound, so the entry forwards to the value's own entry for that key; `=>` (Redirect) sets the `Delegate` to `RedirectLookup<TableType, Path>` along an `@`-path value. The [`delegate_component` AST document](../asts/delegate_component.md) describes each in full.

An **array key** `[A, B]: Provider` expands to one impl pair per bracketed key, all pointing at the same value, because the key evaluates to a vector of evaluated keys that the mapping iterates. A **per-key or per-table generic list** is merged onto every generated impl: a table-level `<'a, T>` is threaded through each impl's generics, and a key may introduce its own extra generics (`<T2> BazKey<T1, T2>`) that merge with the table's.

An **`@`-path key** carries a leading `__Wildcard__` generic and lowers the path to a prefix type ending in that wildcard, which is how a dispatch parameter slots in at lookup time. A **brace group on a path segment** (`@Component.{u32, u64}: P`) expands to one key per element, and the `namespace`/`for` statement forms lower through a shared "for-entry" path that builds a `Namespace<…, Delegate = …>` bound rather than a direct `DelegateComponent` impl; these are the namespace machinery and are detailed in the AST document. A **`for` loop's optional `where` clause** is merged into every impl the loop generates, alongside that reconstructed bound, so a bound written on the loop constrains which keys it wires.

## Error spans

Each generated impl is re-spanned onto the entry that produced it, so a compiler error about that impl points at the entry the user wrote rather than at the whole `delegate_components!` block. The impls are built with `parse_internal!`, which quasi-quotes their tokens; those tokens carry the macro invocation's `call_site` span, and the only tokens with a narrower span are the interpolated key, value, and target type. A coherence conflict (`E0119`) between two entries mapping the same key is reported on the impl header — its `impl` keyword and trait reference — which is exactly the part that starts at `call_site`, so without correction the error underlines the entire block, and two overlapping entries produce two block-wide carets that say nothing about which entry to fix.

`build_delegate_component_impl` and `build_is_provider_for_impl` fix this through `respan_impl`, which re-spans only the impl's two *boundary* tokens — its leading `impl` keyword and its trailing `{ … }` body — onto the entry's diagnostic span, and leaves everything between them alone. That is enough because the compiler derives a generated item's span, and therefore the `E0119` caret, by joining its first and last tokens (`first.to(last)`): with the whole impl at `call_site` the caret is the whole invocation, and pulling just those two ends onto the entry collapses it onto what the user wrote. The interior — the trait reference, the reserved `__Context__`/`__Params__` generics, the key, the wired provider, the target type, a per-entry generic — keeps its own spans.

Leaving the interior in place is not just an economy; it is what keeps the result usable in two ways the whole-impl re-span could not. A diagnostic about a per-entry generic still points where the user declared it — an unconstrained parameter's `E0207` lands on the `<T>` the user wrote rather than being dragged onto the key — because that `<T>` is an interior token the re-span never touches. And a type written in the block stays navigable: rust-analyzer maps a source token to its expanded counterpart purely by source range, ignoring hygiene, so a synthesized reference like `IsProviderFor` or `DelegateComponent` re-spanned onto the key would land on the key's exact range, and go-to-definition on that key would then offer every such collided construct as a candidate. Keeping the interior untouched leaves each synthesized reference at `call_site` (a range no narrower user token shares) and each user token — the wired provider, the target type — at its own span, so navigation resolves to the one right definition. The two tokens that *do* move are a keyword and a delimiter group, never references, so moving them onto the key misleads neither the compiler nor the editor. (An earlier fix re-spanned *every* token of the impl onto the entry; that put `IsProviderFor` and the target type on the key's range and broke go-to-definition, which is why the re-span is now scoped to the boundary.) The [`override_item_span`](../../../crates/macros/cgp-macro-core/src/functions/override_span.rs) helper it uses re-spans a boundary token only when that token is itself synthesized, recognized by its `call_site` span. `check_components!` aims its own error the opposite way, with the unconditional `override_span` helper in the same file, because there the intent *is* to clobber a user token — one shared context type, forced onto each checked component in turn.

The diagnostic span is carried explicitly rather than read back from the generated key, because a key type may be *synthesized* and so no longer carry a useful span. This follows the [`EvaluatedCheckEntry.span`](check_components.md) pattern: `EvaluatedDelegateKey`, `EvaluatedDelegateEntry`, and the `namespace`/`for` intermediary `EvaluatedForEntry` each hold a `span` field, populated at eval time from the token the user actually wrote and threaded through to `respan_impl`. Every entry form sources its span this way, so none falls back to `call_site`: a plain `Key: Provider`, `Key -> Value`, or `Key => @path` mapping (and each element of an array key) takes the key type's own span; the `open` header takes the opened component's span; a `namespace`/`for` statement takes the namespace name or the loop mapping's key; and a nested `UseDelegate<new Inner { .. }>` table lowers through the same per-entry path, so its inner keys are spanned too.

An `@`-path key needs more than its lowered type, since that type is a `PathCons<..>` nest whose first token is a `call_site`-spanned `PathCons` (the same problem `Symbol` avoids by keeping a parse-time span so its synthesized `Chars`/`Symbol` tokens do not fall back to `call_site`). So an `@`-path sources its span from the path segments — the join of the segment spans, which widens to the whole path where the toolchain supports `Span::join` and otherwise keeps the leaf segment (the component of a namespace path, or the dispatch key of an `@Component.key` entry), so the caret lands on the segment that discriminates entries sharing a prefix.

Because the `.stderr` fixtures record the exact line and column of each caret, they double as regression tests for these spans: reverting the re-span snaps the carets back to the block and the snapshots change (see [Tests](#tests)).

## Failure modes

Some `delegate_components!` inputs are accepted by the macro but fail to compile downstream, because the macro lowers each block independently with no whole-program view. These are intended behavior rather than bugs, and their full anatomy is documented in the [error catalog](../../errors/README.md); the [Error spans](#error-spans) section below covers how each caret is aimed at the offending entry.

A **duplicate key** (the same component mapped twice — across blocks, within a block, or an `open` header colliding with a mapping) and an **overlapping generic entry** (a `<T> Wrapper<T>` table conflicting with a specific `Wrapper<u64>`) both produce the coherence error `E0119`, the [conflicting wiring](../../errors/wiring/conflicting-wiring.md) error class.

A **missing impl-side dependency** — a lazily-wired provider whose `where` clause the context cannot satisfy — is accepted here and fails only when the consumer trait is used. Its full anatomy (the `E0599` that names `Person: Greeter<Person>` while *hiding* the missing dependency, and how a `check_components!` site promotes it to a readable error) is documented as the [hidden unsatisfied-dependency](../../errors/hidden/unsatisfied-dependency.md) error class in the error catalog.

An **unconstrained per-entry generic** — a parameter that reaches the provider value but not the key — is rejected with `E0207`, the [unconstrained generic](../../errors/wiring/unconstrained-generic.md) error class.

## Known issues

The macro's parser is permissive about the body shape and surfaces most mistakes as generic `syn` parse errors rather than tailored diagnostics — for example, an `open` header written after a plain mapping fails to parse because statements must lead the block, but the error (`expected `:``) points at the unexpected token rather than explaining the ordering rule.

## Snapshots

Every `snapshot_delegate_components!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint. The basic-delegation snapshots pin the plain-table forms:

- [basic_delegation/delegate_components_macro.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_components_macro.rs) — the canonical `new`-table expansion (two entries) plus the `->` forwarding form that delegates to another table's entry.
- [basic_delegation/delegate_array_key.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_array_key.rs) — an array key expanding to one impl pair per bracketed key.
- [basic_delegation/delegate_generic_table.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_generic_table.rs) — a leading `<'a, T1: Clone>` generic list threaded onto every impl, with a key introducing its own extra generic (`<T2> BazKey<T1, T2>`).

The namespace snapshots pin the statement and `@`-path forms:

- [namespaces/open_dispatch.rs](../../../crates/tests/cgp-tests/tests/namespaces/open_dispatch.rs) — the braced `open { A, B }` header opening two components at once, plus `@Component.Key` per-value entries, including a brace group sharing one provider across several keys.
- [namespaces/multi_param_open.rs](../../../crates/tests/cgp-tests/tests/namespaces/multi_param_open.rs) — the braceless single-component `open Component;` form, dispatched on a multi-segment `@Component.A.B` path with one segment carrying an entry generic.
- [namespaces/namespace_basic.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_basic.rs), [namespaces/namespace_symbol_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_symbol_path.rs), [namespaces/namespace_type_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_type_path.rs) — the `namespace …;` header forwarding every lookup through a namespace trait, with bare, symbol-path, and type-path `@`-keys.
- [namespaces/namespace_multi.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_multi.rs), [namespaces/namespace_group.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_group.rs) — brace-group and array-group `@`-keys expanding to the cartesian product of segments.
- [namespaces/multi_param_namespace.rs](../../../crates/tests/cgp-tests/tests/namespaces/multi_param_namespace.rs) — multi-segment namespace paths with a per-segment generic.
- [namespaces/extended_namespace_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/extended_namespace_wiring.rs) — a namespace table mixing plain and nested-group `@`-paths across several crates' components.
- [namespaces/prefix_default_namespace.rs](../../../crates/tests/cgp-tests/tests/namespaces/prefix_default_namespace.rs) — a `DefaultNamespace` header with fully-qualified `@cgp.core.error.…` paths.
- [namespaces/default_impls_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls_wiring.rs) — the `for <T, Provider> in SomeTable { … }` loop form pulling entries from another lookup table.
- [namespaces/for_where_clause.rs](../../../crates/tests/cgp-tests/tests/namespaces/for_where_clause.rs) — the `for <..> in .. where ..` loop with a `where` clause, pinning the clause merged onto each generated impl beside the reconstructed namespace bound.
- [namespaces/redirect_lookup.rs](../../../crates/tests/cgp-tests/tests/namespaces/redirect_lookup.rs) — a `namespace` header producing the `RedirectLookup`-style blanket `DelegateComponent` impl.
- [dispatching/use_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs) — the legacy `UseDelegate<new … { … }>` nested-table value, including a custom `UseDelegate2` wrapper over tuple keys.

One variant has no snapshot: a bare (non-`new`) single-entry table with a plain type target, distinct from the standalone `new` bundle that owns the canonical snapshot.

## Tests

The behavioral tests confirm the generated wiring resolves and compiles:

- [basic_delegation/delegate_new_struct.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_new_struct.rs) checks that `new` declares the table struct and the table resolves as written.
- [basic_delegation/delegate_new_array_key.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_new_array_key.rs) checks the array-key and nested-`new` forms parse and expand together.
- [basic_delegation/delegate_new_generic_struct.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_new_generic_struct.rs) checks that `<T> new MyComponents<T>` declares a generic table struct.
- [basic_delegation/delegate_nested_use_delegate.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_nested_use_delegate.rs) checks a two-level nested `UseDelegate` value builds an inline dispatch table.
- [basic_delegation/delegate_generic_nested_value.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_generic_nested_value.rs) checks a per-entry `<T>` list threads through both the outer key and the inner generated table struct.
- [basic_delegation/consumer_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/consumer_delegate_getter.rs) and [basic_delegation/consumer_delegate_generic.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/consumer_delegate_generic.rs) check that a context may satisfy some components by wiring and others by a direct trait impl, and that a generic component resolves independently per type argument.

The failure cases in `cgp-macro-tests` pin the attribute rejection:

- [parser_rejections/delegate_components.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/delegate_components.rs) asserts the macro rejects an attribute on the table, on a key, and on a key nested inside a `UseDelegate<new Inner { … }>` value (the last confirms the validator recurses through mapping values rather than dropping the attribute), and that a braceless `open` header listing more than one component is rejected (the braceless form opens exactly one).

The post-codegen compile-fail cases are now `cargo-cgp` UI fixtures that pin the expansions that fail to compile. All are **acceptable** failures — deferred to the compiler by design, none a defect — and their anatomy is documented in the [error catalog](../../errors/README.md); the [Failure modes](#failure-modes) section links each to its class. `cargo-cgp` files each fixture by the *quality of the output* it renders (`acceptable/` when the tool already leads with the cause, `usability/` when the cause is present but buried), which is why the unconstrained-generic case below sits under `usability/` even though it is not a defect:

- [`acceptable/wiring/duplicate-keys/duplicate_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_key.rs) — two blocks mapping the same key expand to conflicting `DelegateComponent` impls (`E0119`).
- [`acceptable/wiring/duplicate-keys/duplicate_key_same_block.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_key_same_block.rs) — the same conflict from two entries in one block; its `.rust.stderr` pins the per-entry [error spans](#error-spans), each caret landing on its own `GreeterComponent` key rather than the whole block.
- [`acceptable/wiring/namespace-paths/delegate_duplicate_path_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/delegate_duplicate_path_key.rs) — the `@`-path analogue: two identical `@cgp.core.error.ErrorTypeProviderComponent` entries under a `namespace` header conflict, and its `.rust.stderr` pins the [error span](#error-spans) landing on the duplicated path leaf rather than the whole block, even though the key lowers to a synthesized `PathCons<..>` type.
- [`acceptable/wiring/duplicate-keys/duplicate_open_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_open_key.rs) — an `open` header colliding with an explicit mapping for the same component; its `.rust.stderr` pins the [error span](#error-spans) of the `open`-header entry, whose span comes from the opened component (a source distinct from the plain key path).
- [`acceptable/wiring/duplicate-keys/overlapping_generic.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/overlapping_generic.rs) — a generic `<T> Wrapper<T>` entry overlaps a specific `Wrapper<u64>` entry at the same key (`E0119`).
- [`acceptable/use-site/missing_dependency.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/use-site/missing_dependency.rs) — a lazily-wired provider whose `Self: HasName` dependency the context does not satisfy; the unmet bound surfaces at the call site (`E0599`). Its anatomy is the [hidden unsatisfied-dependency](../../errors/hidden/unsatisfied-dependency.md) error class.
- [`acceptable/use-site/ordinary_bound_unsatisfied.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/use-site/ordinary_bound_unsatisfied.rs) — the same hidden `E0599` where the suppressed dependency is an *ordinary* trait bound (`Scalar: Eq`, unmet by a wired `f64`) rather than a `HasField`, called rather than checked. Same [hidden unsatisfied-dependency](../../errors/hidden/unsatisfied-dependency.md) class; its surfaced counterpart is the [unsatisfied ordinary trait bound](../../errors/checks/ordinary-trait-bound.md) class.
- [`usability/wiring/constraints/unconstrained_generic.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/constraints/unconstrained_generic.rs) — a per-entry generic that appears only in the value (`<T> GreeterComponent: GreetWith<T>`) lowers to an impl with an unconstrained `T` (`E0207`), which the compiler rejects as it would a hand-written impl.

## Source

- Entry point: `delegate_components` in [cgp-macro-lib/src/delegate_components.rs](../../../crates/macros/cgp-macro-lib/src/delegate_components.rs).
- The table, its entries, keys, values, and statements: [cgp-macro-core/src/types/delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/), documented in [asts/delegate_component.md](../asts/delegate_component.md).
- Attribute rejection is in `validate_attributes.rs`; the impl pair is built in `mapping/eval.rs`.
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
- The `open` and `@`-path forms build on the [`RedirectLookup`](cgp_component.md) impl that `#[cgp_component]` generates.
