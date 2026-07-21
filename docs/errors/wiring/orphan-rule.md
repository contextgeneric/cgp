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

This section describes what plain `cargo check` prints; `cargo-cgp` now reshapes this class into a CGP-framed message (see [How cargo-cgp presents it](#how-cargo-cgp-presents-it)), so what follows is the raw baseline the tool starts from. The compiler reports **`E0210`** — "type parameter `…` must be used as the type parameter for some local type" — naming the impl's uncovered table parameter, with two explanatory notes: "implementing a foreign trait is only possible if at least one of the types for which it is implemented is local," and "only traits defined in the current crate can be implemented for a type parameter." Which parameter is named, and where the caret lands, follows the construct that generated the impl: a `#[default_impl]` names **`__Components__`**, lands the caret on the `#[cgp_impl]` attribute, and attributes the error to the `cgp_impl` macro; a `cgp_namespace!` re-open names **`__Table__`** (the namespace table parameter), lands the caret on the whole `cgp_namespace!` block, and attributes the error to the `cgp_namespace` macro. The parameter differs only because the two constructs name their table parameter differently; the violation is identical. Depending on the exact shape of the generated impl, the sibling orphan error **`E0117`** ("only traits defined in the current crate can be implemented for arbitrary types") can appear instead; both are the orphan rule rejecting the same foreign-trait-for-foreign-type impl.

The rule the compiler is enforcing here is coherence, and understanding it explains why the error frames the fix as a matter of *ownership*. Coherence requires that for any trait and type there is at most one impl, and the orphan rule preserves that across crates by forbidding an impl of a foreign trait unless a local type is *covered* by it — appears before any uncovered type parameter. Were an orphan impl allowed, two unrelated crates could each implement the foreign trait for the foreign type in incompatible ways, and adding a dependency could silently break a build; the orphan rule rejects the impl up front precisely so that can never happen. `E0210` is the specific form of this rule for an impl whose only "type" in the covering position is a bare type parameter (`__Components__`), which no local type covers. The current rule and its wording come from [RFC 2451 (re-rebalancing coherence)](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html); the [`E0210`](../error_codes/e0210.md) reference summarizes it, alongside its sibling [`E0117`](../error_codes/e0117.md).

## Where the root cause is

The mechanical cause is **present** — the error names the foreign trait and the offending type parameter — but the *actionable* cause is CGP-specific and the diagnostic does not state it. What the compiler cannot say is that the impl is foreign because a namespace registration landed on a namespace and key the crate does not own, nor that the remedy is a matter of crate ownership. So while this is not a hidden class, reading the raw `E0210` leaves a user who does not know CGP's namespace mechanics without the fix; the gap is knowledge, not information the compiler withheld.

## How cargo-cgp presents it

`cargo-cgp` reshapes this class into a `[CGP-E011]` message that names the foreign namespace and key rather than the machinery parameter. The header reads `` cannot register the foreign <key> into the foreign namespace `<Namespace>` `` — where `<key>` is `` component `GreeterComponent` `` for a bare marker or `` path `@app.AnnouncerComponent` `` for a prefix path (resugared from its `PathCons<Symbol<…>>` spine) — and the raw coherence notes are replaced by one `help` carrying the ownership-based fix from [Resolving it](#resolving-it) below. The caret still points at the offending `#[cgp_impl]` attribute or `cgp_namespace!` block, and the Rust code stays `E0210`/`E0117`, kept alongside the `[CGP-E011]` tag.

The tool recovers what it names from the compiler, not the error text: it finds the offending impl at the caret, confirms it is a foreign namespace trait — recognized by the single-`Delegate` fingerprint every `cgp_namespace!` trait and the built-in `DefaultNamespace`/`DefaultImpls…` share — implemented for a foreign key, and reads the trigger off the impl's own parameter (`__Table__` for a `cgp_namespace!` re-open, `__Components__` for a `#[default_impl]`/`#[prefix]` registration) so the `help` names the fix that fits: keying the registration on a local component (or registering it from the namespace's crate) for a registration, and inheriting the namespace into a new local one for a re-open. The three triggers are pinned by the fixtures under [`acceptable/wiring/orphan/`](https://github.com/contextgeneric/cargo-cgp/tree/main/tests/ui/acceptable/wiring/orphan); `[CGP-E011]` is catalogued in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

Own one end of the impl. Register the default from the crate that owns the namespace, or key it on a **local** component whose marker the registering crate owns (`#[default_impl(LocalComponent in Namespace)]`), which satisfies the orphan rule because a local type is present. When the wiring genuinely must live downstream of the namespace, use a namespace *body* entry rather than a per-component `#[default_impl]`, per the [namespaces guide](../../guides/namespaces-and-prefixes.md). To *extend* a foreign namespace, do not re-open it with a bare `cgp_namespace!` block — define a **new local namespace that inherits it** (`cgp_namespace! { new Local: ForeignNamespace { … } }`), which is orphan-safe because every emitted impl is for the local namespace trait. See [`DefaultNamespace`](../../reference/traits/default_namespace.md) for the orphan-safe registration patterns.

## Notes for tooling

`cargo-cgp` reshapes this class, as [How cargo-cgp presents it](#how-cargo-cgp-presents-it) describes: it recognizes the shape — a namespace trait (a `#[cgp_namespace]` trait, `DefaultNamespace`, or a `DefaultImpls…`, told apart by their single-`Delegate` fingerprint) implemented for a foreign component marker or a `PathCons<Symbol<…>>` spine — and translates the generic orphan message into the CGP remedy: own one end, by keying the registration on a local component, registering it from the namespace's own crate, or (for a `cgp_namespace!` re-open) inheriting the namespace into a new local one. The raw diagnostic was accurate but framed a CGP wiring decision as a bare coherence rule, so the value the rewrite adds is the translation, not the extraction.

## Backing fixtures

Each fixture sits under `acceptable/wiring/orphan/`, where its `.cgp.stderr` pins the reshaped `[CGP-E011]` output and its `.rust.stderr` the raw `E0210` baseline.

- [`acceptable/wiring/orphan/default_impl_foreign_prefix_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/orphan/default_impl_foreign_prefix_path.rs) — a downstream crate registering a `#[default_impl]` for an upstream *prefixed* component into the upstream namespace, so the key is a foreign `PathCons<Symbol<…>>` path; the reshaped snapshot names the `` path `@app.AnnouncerComponent` `` and the foreign namespace.
- [`acceptable/wiring/orphan/default_impl_foreign_component.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/orphan/default_impl_foreign_component.rs) — the same `#[default_impl]` orphan keyed on a foreign *unprefixed* component marker rather than a path, showing the restriction is not specific to prefixes; the snapshot names the `` component `GreeterComponent` `` instead.
- [`acceptable/wiring/orphan/reopen_foreign_namespace.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/orphan/reopen_foreign_namespace.rs) — a `cgp_namespace!` block without `new` re-opening a foreign namespace; its `__Table__` trigger selects the inherit-a-new-namespace `help`, with the caret on the whole `cgp_namespace!` block.

The orphan-*safe* counterpart is the positive fixture [`ok/cross_crate_wiring.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/ok/cross_crate_wiring.rs), which compiles clean: its auxiliary crates (`cgp-test-crate-a` upstream, `cgp-test-crate-b` downstream) wire a foreign component to a foreign provider, join an upstream namespace, and register a *local* component into the upstream namespace with `#[default_impl]` — every cross-crate impl coherent because one end is always local.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [Overlapping namespace forwarding](namespace-forwarding-conflict.md), [Namespace override conflict](namespace-override-conflict.md), [Namespace inheritance cycle](namespace-inheritance-cycle.md), [Unregistered namespace path](../checks/unregistered-namespace-path.md) — the other namespace-specific failure classes.
- [`DefaultNamespace`](../../reference/traits/default_namespace.md) and [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md) — the namespace mechanics behind the restriction.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0210`/`E0117` entry in the decoder.
