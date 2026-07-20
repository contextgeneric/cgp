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

## The raw diagnostic

This section describes what plain `cargo check` prints — and, for this class, what `cargo-cgp` shows too, since the tool passes the error through unchanged (see [How cargo-cgp presents it](#how-cargo-cgp-presents-it)). The compiler reports **`E0210`** — "type parameter `…` must be used as the type parameter for some local type" — naming the impl's uncovered table parameter, with two explanatory notes: "implementing a foreign trait is only possible if at least one of the types for which it is implemented is local," and "only traits defined in the current crate can be implemented for a type parameter." Which parameter is named, and where the caret lands, follows the construct that generated the impl: a `#[default_impl]` names **`__Components__`**, lands the caret on the `#[cgp_impl]` attribute, and attributes the error to the `cgp_impl` macro; a `cgp_namespace!` re-open names **`__Table__`** (the namespace table parameter), lands the caret on the whole `cgp_namespace!` block, and attributes the error to the `cgp_namespace` macro. The parameter differs only because the two constructs name their table parameter differently; the violation is identical. Depending on the exact shape of the generated impl, the sibling orphan error **`E0117`** ("only traits defined in the current crate can be implemented for arbitrary types") can appear instead; both are the orphan rule rejecting the same foreign-trait-for-foreign-type impl.

The rule the compiler is enforcing here is coherence, and understanding it explains why the error frames the fix as a matter of *ownership*. Coherence requires that for any trait and type there is at most one impl, and the orphan rule preserves that across crates by forbidding an impl of a foreign trait unless a local type is *covered* by it — appears before any uncovered type parameter. Were an orphan impl allowed, two unrelated crates could each implement the foreign trait for the foreign type in incompatible ways, and adding a dependency could silently break a build; the orphan rule rejects the impl up front precisely so that can never happen. `E0210` is the specific form of this rule for an impl whose only "type" in the covering position is a bare type parameter (`__Components__`), which no local type covers. The current rule and its wording come from [RFC 2451 (re-rebalancing coherence)](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html); the [`E0210`](../error_codes/e0210.md) reference summarizes it, alongside its sibling [`E0117`](../error_codes/e0117.md).

## Where the root cause is

The mechanical cause is **present** — the error names the foreign trait and the offending type parameter — but the *actionable* cause is CGP-specific and the diagnostic does not state it. What the compiler cannot say is that the impl is foreign because a namespace registration landed on a namespace and key the crate does not own, nor that the remedy is a matter of crate ownership. So while this is not a hidden class, reading the raw `E0210` leaves a user who does not know CGP's namespace mechanics without the fix; the gap is knowledge, not information the compiler withheld.

## How cargo-cgp presents it

`cargo-cgp` does **not** rewrite this class today: it passes `rustc`'s `E0210` (or `E0117`) through unchanged, uncoded, so the reader sees exactly the raw diagnostic above. This is why the three fixtures sit under `usability/wiring/orphan/` rather than `acceptable/` — the cause is *present* in the output (the error names the foreign trait and the uncovered parameter), but the *actionable* remedy is CGP-specific and the tool does not yet supply it, so each fixture's `.cgp.stderr` matches its `.rust.stderr` byte for byte. The gap is a recognized one, not an oversight: the shape is distinctive enough to key on — an `E0210`/`E0117` whose trait is a namespace trait and whose `Self` is a foreign component marker or a `PathCons<Symbol<…>>` spine — so it is a candidate for a future rewrite into the ownership-framed remedy the [Resolving it](#resolving-it) section states. Until then, no `[CGP-Exxx]` code is stamped; the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md) covers the classes it does rewrite.

## Resolving it

Own one end of the impl. Register the default from the crate that owns the namespace, or key it on a **local** component whose marker the registering crate owns (`#[default_impl(LocalComponent in Namespace)]`), which satisfies the orphan rule because a local type is present. When the wiring genuinely must live downstream of the namespace, use a namespace *body* entry rather than a per-component `#[default_impl]`, per the [namespaces guide](../../guides/namespaces-and-prefixes.md). To *extend* a foreign namespace, do not re-open it with a bare `cgp_namespace!` block — define a **new local namespace that inherits it** (`cgp_namespace! { new Local: ForeignNamespace { … } }`), which is orphan-safe because every emitted impl is for the local namespace trait. See [`DefaultNamespace`](../../reference/traits/default_namespace.md) for the orphan-safe registration patterns.

## Notes for tooling

This is the remaining gap: `cargo-cgp` passes the orphan `E0210`/`E0117` through untranslated (above), so a post-processor that recognizes the shape — a namespace trait (a `#[cgp_namespace]` trait, `DefaultNamespace`, or a `DefaultImpls…`) implemented for a foreign component marker or a `PathCons<Symbol<…>>` spine — could translate the generic orphan message into the CGP remedy: "registering into a foreign namespace needs the crate to own the namespace or a local key; register from the namespace's own crate, key on a local component, use a namespace body entry, or inherit the namespace into a new local one." The raw diagnostic is accurate but frames a CGP wiring decision as a bare coherence rule, so the value the rewrite would add is the translation, not the extraction — which is why the fixtures live in `usability/` until it lands.

## Backing fixtures

Each fixture sits under `usability/wiring/orphan/` because `cargo-cgp` does not yet rewrite the class, so its `.cgp.stderr` and `.rust.stderr` are identical — both pin the raw `E0210`.

- [`usability/wiring/orphan/default_impl_foreign_prefix_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/default_impl_foreign_prefix_path.rs) — a downstream crate registering a `#[default_impl]` for an upstream *prefixed* component into the upstream namespace, so the key is a foreign `PathCons<Symbol<…>>` path; the snapshots pin the `E0210` on `__Components__`, the caret on the `#[cgp_impl]` attribute, and the "implementing a foreign trait" notes.
- [`usability/wiring/orphan/default_impl_foreign_component.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/default_impl_foreign_component.rs) — the same `#[default_impl]` orphan keyed on a foreign *unprefixed* component marker rather than a path, showing the restriction is not specific to prefixes; the snapshots pin the `E0210` on `__Components__`.
- [`usability/wiring/orphan/reopen_foreign_namespace.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/reopen_foreign_namespace.rs) — a `cgp_namespace!` block without `new` re-opening a foreign namespace; the snapshots pin the `E0210` on `__Table__` with the caret on the whole `cgp_namespace!` block, attributed to the `cgp_namespace` macro.

The orphan-*safe* counterpart is the positive fixture [`ok/cross_crate_wiring.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/ok/cross_crate_wiring.rs), which compiles clean: its auxiliary crates (`cgp-test-crate-a` upstream, `cgp-test-crate-b` downstream) wire a foreign component to a foreign provider, join an upstream namespace, and register a *local* component into the upstream namespace with `#[default_impl]` — every cross-crate impl coherent because one end is always local.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [Overlapping namespace forwarding](namespace-forwarding-conflict.md), [Namespace override conflict](namespace-override-conflict.md), [Namespace inheritance cycle](namespace-inheritance-cycle.md), [Unregistered namespace path](../checks/unregistered-namespace-path.md) — the other namespace-specific failure classes.
- [`DefaultNamespace`](../../reference/traits/default_namespace.md) and [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md) — the namespace mechanics behind the restriction.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0210`/`E0117` entry in the decoder.
