# Orphan-rule violation

A generated impl targets a foreign trait and a type built entirely from foreign pieces, which Rust's orphan rule forbids, so the expansion fails with `E0210` (or `E0117`) — most often when a prefixed `#[default_impl]` is registered from a crate that owns neither the namespace nor the path.

## What triggers it

The mistake is registering a per-type default for a *prefixed* component into a namespace from a downstream crate that does not own it. When a component carries `#[prefix(@app in …)]`, its namespace key is a path, so a `#[default_impl(@path in Namespace)]` for it expands to an impl of the foreign namespace trait for a `Self` type built from the `cgp`-owned `PathCons`/`Symbol` spine and the upstream component marker — every element foreign to the registering crate.

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

Rust accepts a foreign-trait impl only when at least one type in it is local, and here none is, so the orphan rule rejects it. This is a whole-program coherence fact CGP cannot see from the macro invocation, so it lowers the impl faithfully and defers to the compiler.

## The diagnostic

The compiler reports **`E0210`** — "type parameter `__Components__` must be used as the type parameter for some local type" — with the caret on the `#[cgp_impl(new …)]` attribute that generated the impl, and two explanatory notes: "implementing a foreign trait is only possible if at least one of the types for which it is implemented is local," and "only traits defined in the current crate can be implemented for a type parameter." A final note attributes the error to the `cgp_impl` attribute macro. Depending on the exact shape of the generated impl, the sibling orphan error **`E0117`** ("only traits defined in the current crate can be implemented for arbitrary types") can appear instead; both are the orphan rule rejecting the same foreign-trait-for-foreign-type impl.

The rule the compiler is enforcing here is coherence, and understanding it explains why the error frames the fix as a matter of *ownership*. Coherence requires that for any trait and type there is at most one impl, and the orphan rule preserves that across crates by forbidding an impl of a foreign trait unless a local type is *covered* by it — appears before any uncovered type parameter. Were an orphan impl allowed, two unrelated crates could each implement the foreign trait for the foreign type in incompatible ways, and adding a dependency could silently break a build; the orphan rule rejects the impl up front precisely so that can never happen. `E0210` is the specific form of this rule for an impl whose only "type" in the covering position is a bare type parameter (`__Components__`), which no local type covers. The current rule and its wording come from [RFC 2451 (re-rebalancing coherence)](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html); the [`E0210` error-index entry](https://doc.rust-lang.org/error_codes/E0210.html) is its reference description.

## Where the root cause is

The mechanical cause is **present** — the error names the foreign trait and the offending type parameter — but the *actionable* cause is CGP-specific and the diagnostic does not state it. What the compiler cannot say is that the impl is foreign because the component is *prefixed* and the namespace lives *upstream*, nor that the remedy is a matter of crate ownership. So while this is not a hidden class, reading the raw `E0210` leaves a user who does not know CGP's namespace mechanics without the fix; the gap is knowledge, not information the compiler withheld.

## Resolving it

Register the default from the crate that owns the namespace, or key the default on a **local** component whose marker is a type the registering crate owns (`#[default_impl(LocalComponent in Namespace)]`), which satisfies the orphan rule because a local type is present in the impl. When the wiring genuinely must live downstream of the namespace, use a namespace *body* entry rather than a per-component `#[default_impl]`, per the [namespaces guide](../../guides/namespaces-and-prefixes.md). See [`DefaultNamespace`](../../reference/traits/default_namespace.md) for the orphan-safe registration patterns.

## Notes for tooling

A `cargo-cgp`-style post-processor should recognize the shape — an `E0210`/`E0117` whose `Self` type is a `PathCons<Symbol<…>>` spine and whose trait is a namespace trait — and translate the generic orphan message into the CGP remedy: "a prefixed `#[default_impl]` for a foreign namespace must live in the namespace's own crate; use a local component key or a namespace body entry to register it downstream." The raw diagnostic is accurate but frames a CGP wiring decision as a bare coherence rule, so the value a tool adds is the translation, not the extraction.

## Backing fixtures

- [acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs) — a downstream crate registering a `#[default_impl]` for an upstream *prefixed* component into the upstream namespace; its `.stderr` pins the `E0210` and the "implementing a foreign trait" notes. The orphan-safe counterpart — a *local* component key registered into a foreign namespace — is exercised in the cross-crate test packages.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [`DefaultNamespace`](../../reference/traits/default_namespace.md) and [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md) — the namespace mechanics behind the restriction.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0210`/`E0117` entry in the decoder.
