# `cgp_namespace!` — implementation

`cgp_namespace!` builds a reusable, inheritable wiring table — a *namespace* — by parsing a `delegate_components!`-style body with a namespace header and emitting a lookup trait, an optional marker struct, and one `impl` of that trait per entry. This document covers how that works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/cgp_namespace.md](../../reference/macros/cgp_namespace.md).

## Entry point

The macro is driven by the thin `cgp_namespace` function in [cgp-macro-lib/src/cgp_namespace.rs](../../../crates/macros/cgp-macro-lib/src/cgp_namespace.rs). Unlike the attribute macros, it is a function-like macro: it parses the whole body into a single `NamespaceTable`, evaluates it, and renders the result.

```rust
let namespace_table: NamespaceTable = parse2(body)?;
Ok(namespace_table.eval()?.to_token_stream())
```

There is no separate attribute to parse — the header (`new`, the namespace name, the optional `: parent`, and the generic list) and the entry table are all part of the one `NamespaceTable`, whose `Parse` impl enforces the grammar. A malformed header or entry is rejected there.

## Pipeline

The macro is a two-step pipeline: parse into a `NamespaceTable`, then `eval` into an `EvaluatedNamespaceTable` that renders itself with `ToTokens`. The [`namespace` AST stack](../asts/namespace.md) documents those types in full.

- **parse** reads the header and the entry table. The entry table is the same `DelegateEntries` type that `delegate_components!` parses, so a namespace body accepts the same mappings, array keys, `open`/`namespace`/`for` statements, and `=>` redirects.
- **eval** builds each output item: the lookup trait (only with `new`), the marker struct (only with `new`), one `impl` per evaluated entry, and — when a parent is named — a leading inheritance `impl`. These are collected into the `EvaluatedNamespaceTable`, which emits them in a fixed order through its `ToTokens`.

## Generated items

A `new` namespace emits, in order, the marker struct `__{Namespace}Components`, the lookup trait `{Namespace}<__Table__>` carrying a single `type Delegate;`, and one `impl {Namespace}<__Table__> for {Key}` per entry; omitting `new` emits only the per-entry impls, so the trait and struct must already be declared elsewhere. When the header names a parent (`: DefaultNamespace`), an inheritance impl is inserted ahead of the entry impls. The struct-then-trait-then-impls order is what the canonical snapshots pin.

Each entry lowers to one impl of the lookup trait, keyed on the entry's key type, whose `Delegate` associated type is the entry's target. A `=>` redirect points the delegate at a `RedirectLookup` along the entry's type-level path, while a plain `:` mapping points it straight at a provider. So a redirect and a direct mapping produce the same shape of impl with a different `Delegate`:

```rust
// new MyNamespace { FooProviderComponent => @MyApp.MyFooComponent, }
impl<__Table__> MyNamespace<__Table__> for FooProviderComponent {
    type Delegate = RedirectLookup<__Table__, PathCons<MyApp, PathCons<MyFooComponent, Nil>>>;
}

// new DefaultShowComponents { [String, u64]: ShowWithDisplay, }
impl<__Table__> DefaultShowComponents<__Table__> for String {
    type Delegate = ShowWithDisplay;
}
```

The `@`-path in a redirect desugars along the same rules as elsewhere in CGP: a lowercase segment (`@my_app`) becomes a `Symbol!` in the `PathCons` spine, an uppercase segment (`@MyApp`) stays a type name, and the segments nest right-to-left into `PathCons<…, Nil>`. An array key expands to one impl per component in the array, all sharing the same `Delegate`.

The **inheritance impl** is what makes a namespace extend a parent. It is built through the same for-entry machinery `delegate_components!` uses, and lowers a parent-namespace clause into a blanket impl that reads each of the parent's entries and re-emits it under the child, so the child inherits every wiring the parent defines:

```rust
// new ExtendedNamespace: DefaultNamespace { }
impl<__Table__, __Key__, __Value__> ExtendedNamespace<__Table__> for __Key__
where
    __Key__: DefaultNamespace<__ExtendedNamespaceComponents>,
    __Key__: DefaultNamespace<__Table__, Delegate = __Value__>,
{
    type Delegate = __Value__;
}
```

A `=>` entry in the child body then layers a *path-prefix rewrite* on top of the inheritance: an entry like `@cgp.core.error => @app` emits an impl keyed on the `PathCons` spine of the source prefix followed by a `__Wildcard__` tail, whose delegate redirects to the same wildcard under the rewritten prefix. This is how a child namespace shadows one branch of an inherited path without touching the rest.

## Behavior and corner cases

The **`new` keyword** is the switch between defining a namespace and extending an existing one. With `new`, the trait and marker struct are emitted; without it, only the entry impls are, and the caller is responsible for having declared the trait. This is why an inheriting namespace and a plain grouping both start with `new` in practice.

The **`__Table__` parameter** threads the caller's own table through every impl, so a namespace is not tied to one context: the lookup trait is generic over `__Table__`, each entry impl carries it, and a `RedirectLookup` delegate re-routes lookups back through it. That is what lets many contexts share one namespace, each supplying its own `__Table__`.

The **reserved identifiers** appear literally in the output. The lookup trait's table parameter is `__Table__`; the inheritance impl introduces `__Key__` and `__Value__`; and a path-prefix rewrite introduces `__Wildcard__` for the inherited tail. The marker struct is always `__{Namespace}Components`, formed by prefixing the namespace identifier with `__` and suffixing `Components`.

The body **reuses the `delegate_components!` grammar wholesale** — array keys, the leading `open { … }` header, `namespace`/`for` statements, and both `:` and `=>` mappings all parse, because the entries are a `DelegateEntries`. Anything `delegate_components!` accepts in a table, a namespace body accepts too.

## Snapshots

Every `snapshot_cgp_namespace!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint:

- [namespaces/namespace_basic.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_basic.rs) — a `new` namespace with one `=>` redirect to a bare component, showing the struct, trait, and single `RedirectLookup` impl over a two-element path.
- [namespaces/namespace_symbol_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_symbol_path.rs) — the same shape with a lowercase leading segment (`@my_app.…`), so the path's head is a `Symbol!` rather than a type name.
- [namespaces/namespace_type_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_type_path.rs) — an uppercase leading segment (`@MyApp.…`) that stays a type name in the `PathCons` spine.
- [namespaces/namespace_multi.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_multi.rs) — two namespaces defined in one module (`MyNamespace` with a type-path head, `OtherNamespace` with a symbol head), confirming distinct marker structs and traits.
- [namespaces/extended.rs](../../../crates/tests/cgp-tests/tests/namespaces/extended.rs) — a `: DefaultNamespace` parent with a `@cgp.core.error => @app` prefix rewrite, capturing both the inheritance blanket impl and the wildcard-tail path-rewrite impl.
- [namespaces/default_impls.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls.rs) — an array-key direct-mapping namespace (`[String, u64]: ShowWithDisplay`) expanding to one impl per key, and an empty `: DefaultNamespace` extension emitting only the inheritance impl.

One variant has no dedicated `snapshot_cgp_namespace!` yet: a namespace body carrying a `namespace` or `for` statement (as opposed to plain mappings and `=>` redirects), which is exercised only through the wiring tests below.

## Tests

The behavioral tests confirm the generated namespaces wire correctly:

- [namespaces/namespace_group.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_group.rs) wires a context to a namespace and checks the grouped components resolve.
- [namespaces/extended_namespace_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/extended_namespace_wiring.rs) checks that an extended namespace inherits its parent's entries and that the child's prefix rewrite takes effect.
- [namespaces/default_impls_wiring.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls_wiring.rs) checks that a `DefaultNamespace`-based namespace supplies defaults a context can override.
- [namespaces/multi_param_namespace.rs](../../../crates/tests/cgp-tests/tests/namespaces/multi_param_namespace.rs) and [namespaces/prefix_default_namespace.rs](../../../crates/tests/cgp-tests/tests/namespaces/prefix_default_namespace.rs) exercise parameterized namespaces and the `#[prefix(...)]` join that attaches a component to one.

## Source

- Entry point: `cgp_namespace` in [cgp-macro-lib/src/cgp_namespace.rs](../../../crates/macros/cgp-macro-lib/src/cgp_namespace.rs).
- Pipeline and its AST types: [cgp-macro-core/src/types/namespace/](../../../crates/macros/cgp-macro-core/src/types/namespace/), documented in [asts/namespace.md](../asts/namespace.md).
- The entry table reuses the `DelegateEntries` grammar from [cgp-macro-core/src/types/delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/); the inheritance impl comes from `inherit.rs`.
- The `RedirectLookup` marker and every generated fragment are built with [parse_internal!](../macros/parse_internal.md).
- The `#[prefix(...)]` attribute that attaches a component to a namespace is handled as part of `#[cgp_component]`; see [entrypoints/cgp_component.md](cgp_component.md).
