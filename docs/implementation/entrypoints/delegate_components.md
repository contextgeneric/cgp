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

All real logic lives in `cgp-macro-core`. A malformed body fails while parsing `DelegateTable`, and an attribute on the table or any key fails in `validate_attributes` with a spanned "unsupported attribute" error rather than being silently dropped.

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
// open { AreaCalculatorComponent };  →  the redirect entry
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

An **`@`-path key** carries a leading `__Wildcard__` generic and lowers the path to a prefix type ending in that wildcard, which is how a dispatch parameter slots in at lookup time. A **brace group on a path segment** (`@Component.{u32, u64}: P`) expands to one key per element, and the `namespace`/`for` statement forms lower through a shared "for-entry" path that builds a `Namespace<…, Delegate = …>` bound rather than a direct `DelegateComponent` impl; these are the namespace machinery and are detailed in the AST document.

## Known issues

The macro's parser is permissive about the body shape and surfaces most mistakes as generic `syn` parse errors rather than tailored diagnostics — for example, an `open` header written after a plain mapping fails to parse because statements must lead the block, but the error points at the unexpected token rather than explaining the ordering rule. There is no failure-case coverage for the delegate family in `cgp-macro-tests`.

## Snapshots

Every `snapshot_delegate_components!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint. The basic-delegation snapshots pin the plain-table forms:

- [basic_delegation/delegate_components_macro.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_components_macro.rs) — the canonical `new`-table expansion (two entries) plus the `->` forwarding form that delegates to another table's entry.
- [basic_delegation/delegate_array_key.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_array_key.rs) — an array key expanding to one impl pair per bracketed key.
- [basic_delegation/delegate_generic_table.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/delegate_generic_table.rs) — a leading `<'a, T1: Clone>` generic list threaded onto every impl, with a key introducing its own extra generic (`<T2> BazKey<T1, T2>`).

The namespace snapshots pin the statement and `@`-path forms:

- [namespaces/open_dispatch.rs](../../../crates/tests/cgp-tests/tests/namespaces/open_dispatch.rs) — the `open { … }` header plus `@Component.Key` per-value entries, including a brace group sharing one provider across several keys.
- [namespaces/multi_param_open.rs](../../../crates/tests/cgp-tests/tests/namespaces/multi_param_open.rs) — an `open` dispatch on a multi-segment `@Component.A.B` path, one segment carrying an entry generic.
- [namespaces/namespace_basic.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_basic.rs), [namespaces/namespace_symbol_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_symbol_path.rs), [namespaces/namespace_type_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_type_path.rs) — the `namespace …;` header forwarding every lookup through a namespace trait, with bare, symbol-path, and type-path `@`-keys.
- [namespaces/namespace_multi.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_multi.rs), [namespaces/namespace_group.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_group.rs) — brace-group and array-group `@`-keys expanding to the cartesian product of segments.
- [namespaces/multi_param_namespace.rs](../../../crates/tests/cgp-tests/tests/namespaces/multi_param_namespace.rs) — multi-segment namespace paths with a per-segment generic.
- [namespaces/extended_namespace_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/extended_namespace_wiring.rs) — a namespace table mixing plain and nested-group `@`-paths across several crates' components.
- [namespaces/prefix_default_namespace.rs](../../../crates/tests/cgp-tests/tests/namespaces/prefix_default_namespace.rs) — a `DefaultNamespace` header with fully-qualified `@cgp.core.error.…` paths.
- [namespaces/default_impls_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls_wiring.rs) — the `for <T, Provider> in SomeTable { … }` loop form pulling entries from another lookup table.
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

## Source

- Entry point: `delegate_components` in [cgp-macro-lib/src/delegate_components.rs](../../../crates/macros/cgp-macro-lib/src/delegate_components.rs).
- The table, its entries, keys, values, and statements: [cgp-macro-core/src/types/delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/), documented in [asts/delegate_component.md](../asts/delegate_component.md).
- Attribute rejection is in `validate_attributes.rs`; the impl pair is built in `mapping/eval.rs`.
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
- The `open` and `@`-path forms build on the [`RedirectLookup`](cgp_component.md) impl that `#[cgp_component]` generates.
