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

**Attributes are rejected**, matching `delegate_components!`. The entry grammar parses `#[..]` attributes on keys (a `DelegateEntries` reuses the same key parsers), but no attribute is supported, so the entry point calls `entries.validate_attributes()` after parsing to fail with a spanned "unsupported attribute" error rather than silently dropping one. The check recurses through the statement forms, so an attribute on a key inside a `for` loop is rejected too.

A **`for` loop's optional `where` clause** is applied. `ForDelegateStatement` parses a `WhereClause?` after the `in` table, and `eval_for_entries` merges its predicates into every generated impl alongside the reconstructed `__Key__: Namespace<…, Delegate = __Value__>` bound, so a bound written on the loop constrains which keys it wires. This applies wherever a `for` loop appears — a namespace body or a context's `delegate_components!` table.

## Error spans

Each generated namespace impl is re-spanned onto the entry that produced it, so a coherence conflict (`E0119`) between two entries mapping the same key points at the offending entry rather than at the whole `cgp_namespace!` block. The impls are built with `parse_internal!`, whose tokens carry the macro's `call_site` span; `build_namespace_impl` therefore re-spans each finished impl's boundary tokens (its `impl` keyword and `{ … }` body) onto the entry's `span` — leaving the interior, including the mapped value and any per-entry generic, at its own spans — the same `respan_impl` technique the `DelegateComponent`/`IsProviderFor` impls use — see [delegate_components.md](delegate_components.md#error-spans). This matters most for a `=>` prefix-rewrite entry, whose key type is a synthesized `PathCons<..>` nest at `call_site`; the entry instead sources its span from the path segments the user wrote, so the caret lands on the duplicated path leaf. The `#[prefix(...)]` attribute impl is built separately (`PrefixAttribute::to_namespace_impl`) and keyed on the component name, which already carries the trait's own span.

## Failure modes

Some namespace input is accepted by the macro but fails to compile downstream, deferred to the compiler because it lacks the whole-program view the check needs. Each is intended behavior, not a bug, and its full anatomy lives in the [error catalog](../../errors/README.md); the [Error spans](#error-spans) section below covers how each caret is aimed at the offending entry.

Three distinct namespace mistakes all surface as `E0119`, and the catalog splits them by usage rather than by the shared error code. A **duplicate key** — the same key mapped twice in one `cgp_namespace!` block — is a plain [conflicting wiring](../../errors/wiring/conflicting-wiring.md). **Overriding a key a namespace already claims** — a context wiring a path the joined namespace registers, or a child namespace redefining a key it inherits — is the [namespace override conflict](../../errors/wiring/namespace-override-conflict.md): a `namespace N;` header emits a blanket `impl<Key> DelegateComponent<Key> for Ctx where Key: N<Ctx>` covering every path `N` resolves, and the inheritance impl covers every key the parent binds, so a specific entry for a claimed key overlaps one of them; to override, target a path the namespace routes *to* but does not terminate. **Emitting two blanket forwardings** — joining two namespaces, or a bare-key `for <Key, Value> in Table { Key: Value }` loop alongside a `namespace` join — is the [overlapping namespace forwarding](../../errors/wiring/namespace-forwarding-conflict.md): each covers every key, so the two overlap, which is why a context joins one namespace and a loop key must sit inside a path.

**Registering into a namespace the crate does not own, keyed on a type it does not own**, fails the orphan rule (`E0210`) — the [orphan-rule violation](../../errors/wiring/orphan-rule.md) error class. This covers a `#[default_impl]` on a *prefixed* component (`impl Namespace<_> for PathCons<..>`, offending parameter `__Components__`), a `#[default_impl]` keyed on a foreign *unprefixed* component marker (`impl Namespace<_> for GreeterComponent`), and a `cgp_namespace!` block *without* `new` re-opening a foreign namespace (`impl ForeignNamespace<_> for Key`, offending parameter `__Table__`, caret on the whole block) — each an impl of a foreign trait for a foreign type. The orphan-safe alternatives are owning either end: register from the namespace's crate, key on a local component, use a namespace body entry, or inherit the namespace into a new local one.

**Joining a namespace that routes a component to a path no entry binds** produces an `E0277` when the wiring is checked — the [unregistered namespace path](../../errors/checks/unregistered-namespace-path.md) error class. A `#[prefix]` registers the *routing* (the namespace resolves the component to a `RedirectLookup` along the path) but not a provider at the path's leaf; if nothing (a `#[default_impl]`, a body entry, or a direct `@path:` line) ever binds one, the `RedirectLookup` finds no `DelegateComponent` entry and the lookup fails. This is a *lookup* failure, distinct from an unsatisfied *dependency*.

**A circular namespace parent chain** (`new A: B` together with `new B: A`, or a self-inheriting `new A: A`) overflows the trait solver with `E0275` — the [namespace inheritance cycle](../../errors/wiring/namespace-inheritance-cycle.md) error class. The inheritance blanket impl's `where` clause requires the parent's lookup trait, so a looping parent chain yields a `where` clause that never discharges; unlike the lazy `UseContext` [wiring cycle](../../errors/wiring/wiring-cycle.md), the compiler catches it *eagerly* while checking each generated inheritance impl, so both `cgp_namespace!` definitions carry the overflow with no use site required.

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
- [namespaces/for_where_clause.rs](../../../crates/tests/cgp-tests/tests/namespaces/for_where_clause.rs) pins the `for <..> in .. where ..` loop's `where` clause landing on each generated impl (a `snapshot_delegate_components!`, since `delegate_components!` owns the `for`-loop form).
- [namespaces/default_impl_use_type.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impl_use_type.rs) registers a provider with a `#[use_type]` abstract-type dependency into a namespace via `#[default_impl]` and resolves it through a context that joins the namespace. Its `snapshot_cgp_impl!` pins that the namespace-registration impl carries no `where` clause — the provider's impl-side bounds must not leak onto the registration impl's `Self` (the path key), or the abstract-type dependency would be demanded of the path rather than the context. See [asts/attributes/default_impl.md](../asts/attributes/default_impl.md) for the mechanism.

The cross-crate coverage confirming the orphan-safe direction of the default-impl coherence rule now lives in `cargo-cgp` (its auxiliary crates `cgp-test-crate-a`/`-b`, migrated out of this repository):

- [`ok/cross_crate_wiring.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/ok/cross_crate_wiring.rs) builds `cgp-test-crate-b`, which registers a *local* component into `cgp-test-crate-a`'s foreign `AppNamespace` with `#[default_impl(FarewellComponent in AppNamespace)]` — legal because the crate owns the component key — and resolves it through a local context that joins the namespace. Its failing orphan-rule counterparts are the `usability/wiring/orphan/` fixtures cataloged under [orphan-rule](../../errors/wiring/orphan-rule.md).

The rejection cases in `cgp-macro-tests` pin the attribute rejection:

- [parser_rejections/cgp_namespace.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/cgp_namespace.rs) asserts the macro rejects an attribute on a `:` mapping key, on a `=>` redirect key, and on a key inside a `for` loop.

The post-codegen compile-fail cases are now `cargo-cgp` UI fixtures that pin accepted expansions failing to compile — all **acceptable** failures (deferred to the compiler by design, none a defect), documented in the [error catalog](../../errors/README.md); the [Failure modes](#failure-modes) section links each to its class. `cargo-cgp` files each fixture by the *quality of the output* it renders (`acceptable/` when it already leads with the cause, `usability/` when the cause is present but buried), which is why the orphan-rule cases below sit under `usability/` even though none is a defect:

- [`acceptable/wiring/namespace-paths/namespace_duplicate_path_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/namespace_duplicate_path_key.rs) — two identical `@`-path prefix-rewrite entries conflict (`E0119`); its `.rust.stderr` pins the [error span](#error-spans) landing on the duplicated path leaf rather than the whole block, even though the key lowers to a synthesized `PathCons<..>` type.
- [`acceptable/wiring/namespace-paths/override_registered_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/override_registered_path.rs) — a context joins a namespace that registers a path (via `#[default_impl]`) and also wires that path directly, so the `namespace` blanket impl and the direct entry both implement `DelegateComponent` for the path (`E0119`, with the `downstream crates may implement` note); the context-level shape of the [namespace override conflict](../../errors/wiring/namespace-override-conflict.md) class.
- [`acceptable/wiring/namespace-paths/inherited_override_conflict.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/inherited_override_conflict.rs) — a child namespace that redefines a key its parent binds, so the inheritance blanket impl and the child's specific entry overlap (a *single* `E0119` on `ChildNs<_>` for the component marker, no note); the namespace-level shape of the [namespace override conflict](../../errors/wiring/namespace-override-conflict.md) class.
- [`acceptable/wiring/duplicate-keys/for_loop_bare_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/for_loop_bare_key.rs) — a bare-key `for` loop alongside a `namespace` join, whose two blanket `DelegateComponent`/`IsProviderFor` impls overlap (`E0119`, fully-generic carets, no downstream note); the [overlapping namespace forwarding](../../errors/wiring/namespace-forwarding-conflict.md) class.
- [`acceptable/wiring/namespace-paths/two_namespaces_joined.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/two_namespaces_joined.rs) — two `namespace` joins on one context, each emitting a blanket forwarding impl over every key, which overlap (`E0119`, fully-generic carets); the same [overlapping namespace forwarding](../../errors/wiring/namespace-forwarding-conflict.md) class.
- [`usability/wiring/orphan/default_impl_foreign_prefix_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/default_impl_foreign_prefix_path.rs) — a downstream crate registers a default for an upstream *prefixed* component into the upstream namespace, whose `impl Namespace<_> for PathCons<..>` is wholly foreign and violates the orphan rule (`E0210` on `__Components__`); the [orphan-rule violation](../../errors/wiring/orphan-rule.md) class. A cross-crate case: an `//@aux-build` upstream crate supplies the component and namespace.
- [`usability/wiring/orphan/default_impl_foreign_component.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/default_impl_foreign_component.rs) — the same orphan keyed on a foreign *unprefixed* component marker rather than a path (`E0210` on `__Components__`); shows the restriction is not specific to prefixes. Same [orphan-rule violation](../../errors/wiring/orphan-rule.md) class.
- [`usability/wiring/orphan/reopen_foreign_namespace.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/reopen_foreign_namespace.rs) — a `cgp_namespace!` block without `new` re-opening a foreign namespace, whose entry impl is foreign (`E0210` on `__Table__`, caret on the whole block); same [orphan-rule violation](../../errors/wiring/orphan-rule.md) class.
- [`acceptable/resolution/unregistered_prefix_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/resolution/unregistered_prefix_path.rs) — a `#[prefix]`-ed component joined through its namespace with no provider bound at its path, so a `check_components!` reports the redirect target's `DefaultNamespace`/`DelegateComponent` lookup as unsatisfied (`E0277`); the [unregistered namespace path](../../errors/checks/unregistered-namespace-path.md) class.

The **namespace inheritance cycle** is the one class with no `cargo-cgp` fixture: under cargo-cgp's next-generation trait solver the two mutually-inheriting namespaces compile clean, with no `E0275` left to snapshot — a *missing*-error divergence (the reverse of the usual next-solver caveat) that makes this the single un-migrated class, recorded as such in [cargo-cgp's tests/README.md](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md). The eager `E0275` overflow at both `cgp_namespace!` definitions remains a plain-`rustc` behavior, described in the [namespace inheritance cycle](../../errors/wiring/namespace-inheritance-cycle.md) class.

## Source

- Entry point: `cgp_namespace` in [cgp-macro-lib/src/cgp_namespace.rs](../../../crates/macros/cgp-macro-lib/src/cgp_namespace.rs).
- Pipeline and its AST types: [cgp-macro-core/src/types/namespace/](../../../crates/macros/cgp-macro-core/src/types/namespace/), documented in [asts/namespace.md](../asts/namespace.md).
- The entry table reuses the `DelegateEntries` grammar from [cgp-macro-core/src/types/delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/); the inheritance impl comes from `inherit.rs`.
- The `RedirectLookup` marker and every generated fragment are built with [parse_internal!](../macros/parse_internal.md).
- The `#[prefix(...)]` attribute that attaches a component to a namespace is handled as part of `#[cgp_component]`; see [entrypoints/cgp_component.md](cgp_component.md).
