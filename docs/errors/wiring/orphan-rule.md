# Orphan-rule violation

A generated impl registers a component into a *foreign* namespace with no local type anywhere in the impl, which Rust's orphan rule forbids, so the expansion fails with `E0210` (or `E0117`) — the failure a crate hits when it tries to add namespace wiring for a namespace, component, or path it does not own.

## What triggers it

The mistake is registering into a namespace whose *trait* the crate does not own, keyed on a type the crate also does not own — an impl of a foreign trait for a foreign type. A namespace registration lowers to `impl<Param> Namespace<Param> for Key`, and Rust accepts a foreign-trait impl only when a local type covers its parameters. When both the namespace and the key are foreign, nothing is local, so the orphan rule rejects it. Three constructs register into a namespace, and each hits the rule the same way when nothing local is in reach.

The first is a **prefixed `#[default_impl]`**, where the component carries `#[prefix(@app in …)]` so its namespace key is a path built from the `cgp`-owned `PathCons`/`Symbol` spine and the upstream component marker — every element foreign to a downstream crate:

```rust
// In a downstream crate, for an upstream prefixed component `Announcer`:
#[cgp_impl(new AnnounceQuietly)]
#[default_impl(@app.AnnouncerComponent in AppNamespace)] // AppNamespace + path all foreign
impl Announcer
where
    Self: HasName,
{
    fn announce(&self) -> String { format!("(psst, {})", self.name()) }
}
```

The second is an **unprefixed `#[default_impl]` keyed on a foreign component marker**, which shows the restriction is not about prefixes: `#[default_impl(GreeterComponent in AppNamespace)]` for a foreign, unprefixed `GreeterComponent` expands to `impl<Components> AppNamespace<Components> for GreeterComponent` — a bare foreign marker as the key, still foreign trait for foreign type.

The third is a **`cgp_namespace!` block without `new`** that re-opens a foreign namespace to add an entry. Omitting `new` tells the macro the namespace trait exists elsewhere and to emit only the entry impls, so `cgp_namespace! { AppNamespace { GreeterComponent => @foo } }` for a foreign `AppNamespace` emits `impl<Table> AppNamespace<Table> for GreeterComponent { type Delegate = … }` — again foreign trait for foreign key.

Every form reduces to the same fact: a foreign namespace trait implemented for a foreign key, with no local type to satisfy the orphan rule. This is a whole-program coherence fact CGP cannot see from the macro invocation, so it lowers the impl faithfully and defers to the compiler. The orphan-*safe* counterpart is owning *either* end — a crate may register a *local* component's marker into a foreign namespace (the key is local), or add entries to a namespace whose trait it owns; only when both are foreign does the impl become an orphan.

## The diagnostic

The compiler reports **`E0210`** — "type parameter `…` must be used as the type parameter for some local type" — naming the impl's uncovered table parameter, with two explanatory notes: "implementing a foreign trait is only possible if at least one of the types for which it is implemented is local," and "only traits defined in the current crate can be implemented for a type parameter." Which parameter is named, and where the caret lands, follows the construct that generated the impl: a `#[default_impl]` names **`__Components__`**, lands the caret on the `#[cgp_impl]` attribute, and attributes the error to the `cgp_impl` macro; a `cgp_namespace!` re-open names **`__Table__`** (the namespace table parameter), lands the caret on the whole `cgp_namespace!` block, and attributes the error to the `cgp_namespace` macro. The parameter differs only because the two constructs name their table parameter differently; the violation is identical. Depending on the exact shape of the generated impl, the sibling orphan error **`E0117`** ("only traits defined in the current crate can be implemented for arbitrary types") can appear instead; both are the orphan rule rejecting the same foreign-trait-for-foreign-type impl.

The rule the compiler is enforcing here is coherence, and understanding it explains why the error frames the fix as a matter of *ownership*. Coherence requires that for any trait and type there is at most one impl, and the orphan rule preserves that across crates by forbidding an impl of a foreign trait unless a local type is *covered* by it — appears before any uncovered type parameter. Were an orphan impl allowed, two unrelated crates could each implement the foreign trait for the foreign type in incompatible ways, and adding a dependency could silently break a build; the orphan rule rejects the impl up front precisely so that can never happen. `E0210` is the specific form of this rule for an impl whose only "type" in the covering position is a bare type parameter (`__Components__`), which no local type covers. The current rule and its wording come from [RFC 2451 (re-rebalancing coherence)](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html); the [`E0210`](../error_codes/e0210.md) reference summarizes it, alongside its sibling [`E0117`](../error_codes/e0117.md).

## Where the root cause is

The mechanical cause is **present** — the error names the foreign trait and the offending type parameter — but the *actionable* cause is CGP-specific and the diagnostic does not state it. What the compiler cannot say is that the impl is foreign because a namespace registration landed on a namespace and key the crate does not own, nor that the remedy is a matter of crate ownership. So while this is not a hidden class, reading the raw `E0210` leaves a user who does not know CGP's namespace mechanics without the fix; the gap is knowledge, not information the compiler withheld.

## Resolving it

Own one end of the impl. Register the default from the crate that owns the namespace, or key it on a **local** component whose marker the registering crate owns (`#[default_impl(LocalComponent in Namespace)]`), which satisfies the orphan rule because a local type is present. When the wiring genuinely must live downstream of the namespace, use a namespace *body* entry rather than a per-component `#[default_impl]`, per the [namespaces guide](../../guides/namespaces-and-prefixes.md). To *extend* a foreign namespace, do not re-open it with a bare `cgp_namespace!` block — define a **new local namespace that inherits it** (`cgp_namespace! { new Local: ForeignNamespace { … } }`), which is orphan-safe because every emitted impl is for the local namespace trait. See [`DefaultNamespace`](../../reference/traits/default_namespace.md) for the orphan-safe registration patterns.

## Notes for tooling

A `cargo-cgp`-style post-processor should recognize the shape — an `E0210`/`E0117` whose trait is a namespace trait (a `#[cgp_namespace]` trait, `DefaultNamespace`, or a `DefaultImpls…`) and whose `Self` type is a foreign component marker or a `PathCons<Symbol<…>>` spine — and translate the generic orphan message into the CGP remedy: "registering into a foreign namespace needs the crate to own the namespace or a local key; register from the namespace's own crate, key on a local component, use a namespace body entry, or inherit the namespace into a new local one." The raw diagnostic is accurate but frames a CGP wiring decision as a bare coherence rule, so the value a tool adds is the translation, not the extraction.

## Backing fixtures

- [acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs) — a downstream crate registering a `#[default_impl]` for an upstream *prefixed* component into the upstream namespace, so the key is a foreign `PathCons<Symbol<…>>` path; its `.stderr` pins the `E0210` on `__Components__` and the "implementing a foreign trait" notes.
- [acceptable/cgp_namespace/default_impl_foreign_component.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/default_impl_foreign_component.rs) — the same `#[default_impl]` orphan keyed on a foreign *unprefixed* component marker rather than a path, showing the restriction is not specific to prefixes; its `.stderr` pins the `E0210` on `__Components__`.
- [acceptable/cgp_namespace/reopen_foreign_namespace.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/reopen_foreign_namespace.rs) — a `cgp_namespace!` block without `new` re-opening a foreign namespace; its `.stderr` pins the `E0210` on `__Table__` with the caret on the whole `cgp_namespace!` block, attributed to the `cgp_namespace` macro.

The orphan-*safe* counterparts — a *local* component key registered into the foreign `AppNamespace`, and a local context wiring a foreign component — are exercised in the cross-crate test packages (`cgp-test-crate-b`).

## Related

- [Conflicting wiring](conflicting-wiring.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [Overlapping namespace forwarding](namespace-forwarding-conflict.md), [Namespace override conflict](namespace-override-conflict.md), [Namespace inheritance cycle](namespace-inheritance-cycle.md), [Unregistered namespace path](../checks/unregistered-namespace-path.md) — the other namespace-specific failure classes.
- [`DefaultNamespace`](../../reference/traits/default_namespace.md) and [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md) — the namespace mechanics behind the restriction.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0210`/`E0117` entry in the decoder.
