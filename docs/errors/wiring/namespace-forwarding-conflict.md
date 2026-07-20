# Overlapping namespace forwarding

A context's wiring emits two *blanket* forwarding impls that each cover every component key — by joining two namespaces, or by joining one namespace alongside a bare-key `for` loop — and the compiler rejects the overlap with `E0119`.

## What triggers it

This class arises when a single [`delegate_components!`](../../reference/macros/delegate_components.md) block produces more than one blanket [`DelegateComponent`](../../reference/traits/delegate_component.md) impl over the *whole* key space. A `namespace N;` header forwards every unresolved key through the namespace `N`, so it lowers to a blanket `impl<Key, Value> DelegateComponent<Key> for Ctx where Key: N<Ctx, …>` (paired with the matching [`IsProviderFor`](../../reference/traits/is_provider_for.md) forwarding). Two such headers — or one header plus a bare-key `for` loop, which lowers to the same all-keys shape — produce two blanket impls that overlap, because a key could satisfy both `where` clauses at once.

The first form is **joining two namespaces on one context**:

```rust
delegate_components! {
    App {
        namespace NamespaceA;
        namespace NamespaceB; // second blanket DelegateComponent<Key> impl — overlaps the first
    }
}
```

The second form is **a bare-key `for` loop beside a namespace join**, where the loop wires `Key` directly rather than embedding it in a path:

```rust
delegate_components! {
    App {
        namespace DefaultNamespace;

        for <Key, Value> in GreeterTable {
            Key: Value,          // blanket DelegateComponent<Key> impl — overlaps the namespace's
        }
    }
}
```

Both reduce to the same mistake: two impls that each claim every key. A context can forward through at most one namespace, and a `for` loop's key must sit inside a path (`@app.SomeComponent.Key: Value`) so it keys a *concrete* path rather than every key. CGP lowers each blanket impl faithfully and cannot see from one block that two of them span the same keys, so it defers the overlap to the compiler.

## The raw diagnostic

This section describes what plain `cargo check` prints — the fallback when `cargo-cgp` is not on hand; [How cargo-cgp presents it](#how-cargo-cgp-presents-it) below covers the readable form. This is a **structural** class reported as a pair of **[`E0119`](../error_codes/e0119.md) conflicting implementations**, one for each trait a namespace join forwards — the `DelegateComponent<_>` table impl and the `IsProviderFor<_, _, _>` forwarding impl that keeps dependency errors diagnosable through [checks](../../reference/macros/check_components.md). Both are printed with fully-generic `DelegateComponent<_>` / `IsProviderFor<_, _, _>` types, and each carries the two-caret shape: "first implementation here" on the first `namespace` header, and "conflicting implementation for `App`" on the second header (or on the loop's `Key: Value` line). The carets land on the entries the user wrote, so the diagnostic points straight at the two lines to reconcile.

The signature of this class is that **both** conflicting types are *fully generic* — `DelegateComponent<_>` and `IsProviderFor<_, _, _>`, with no concrete key — and the conflict carries **no** "downstream crates may implement …" note. That is what separates it from the [namespace override conflict](namespace-override-conflict.md), where one side of the overlap names a *specific* key (a path or a component marker). The generic-versus-generic shape follows from how Rust's coherence checker reasons here. Coherence forbids two impls that could both apply to one type, and both impls have the form `impl<Key, Value> DelegateComponent<Key> for App where Key: SomeNamespace<App, …>`: the self type is the same local `App`, and the key is a free parameter bounded only by a namespace trait. Nothing prevents a single key type from implementing both namespace traits, so the compiler proves the overlap *directly*, from the impls in front of it — it needs no hypothetical future impl to construct the conflict, which is why no downstream note appears. The [conflicting wiring](conflicting-wiring.md) class carries the full account of `E0119` and the RFC 2451 coherence reasoning this builds on; the point here is only that a note-free `E0119` on generic `DelegateComponent<_>`/`IsProviderFor<_, _, _>` is this class's signature.

## Where the root cause is

The root cause is **present and precise**: the two carets name the two overlapping entries directly, and the fully-generic trait types confirm the overlap spans every key. This is a structural class with no note chain to walk and nothing suppressed, so a reader trusts the carets and reconciles the two lines. The only reading the diagnostic does not supply is the CGP-specific fix — that a context forwards through one namespace, not several — because the message describes the collision in impl terms rather than in terms of the wiring intent.

## How cargo-cgp presents it

`cargo-cgp` recognizes this class and rewrites it to state the intent the raw impl-level collision hides. It keeps the `E0119` code, collapses the `DelegateComponent<_>` + `IsProviderFor<_, _, _>` pair into one message (suppressing the redundant `IsProviderFor` half), and stamps **`[CGP-E006]` multiple namespaces** — `` [CGP-E006] only one namespace can be used for each target type in `delegate_components!`, but `App` uses both `NamespaceA` and `NamespaceB` ``. The two `rustc` carets are kept, landing on the two `namespace` headers (or, for the loop form, on the second header and the loop's `Key: Value` line), because they already point at the lines to reconcile; there is no `root cause:` tree, this being a structural class. The bare-key `for` loop form produces the *same* `[CGP-E006]` headline, naming the loop's table (`GreeterTable`) as the second "namespace", which is exactly right — a bare-key loop forwards over every key just as a namespace join does. The code is defined in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

Emit only one blanket forwarding impl. When two namespaces are joined, keep a single `namespace N;` and fold the other namespace's entries into `N` through *inheritance* — define one namespace that inherits the rest (`cgp_namespace! { new Combined: NamespaceA { … } }`, itself inheriting further) and join that one, per the [namespaces guide](../../guides/namespaces-and-prefixes.md). When a bare-key `for` loop collides with a namespace join, embed the loop's key in a path (`@app.SomeComponent.Key: Value`) so it keys a concrete dispatch path instead of every key — which is also why a `for` loop is the natural tool for a generic-parameter component, whose dispatch path *is* the loop key. Either fix leaves the context with one forwarding impl and the overlap gone.

## Notes for tooling

`cargo-cgp` already recognizes and rewrites this class in full (above), leading with the `[CGP-E006]` wiring-intent headline rather than the impl-level collision. The one signal it keys on is worth restating for any post-processor: the absent "downstream crates may implement …" note over fully-generic `DelegateComponent<_>`/`IsProviderFor<_, _, _>` types is what distinguishes this class from the [namespace override conflict](namespace-override-conflict.md), and so routes the collision to `[CGP-E006]` rather than the override's `[CGP-E005]`.

## Backing fixtures

The `.rust.stderr` snapshot pins the raw generic `E0119` pair and the `.cgp.stderr` the `[CGP-E006]` reshaping.

- [`wiring/namespace-paths/two_namespaces_joined.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/two_namespaces_joined.rs) — two `namespace` joins on one context; the `.rust.stderr` pins the `E0119` pair with carets on the two `namespace` lines and no downstream note, the `.cgp.stderr` the single `[CGP-E006]` headline naming both `NamespaceA` and `NamespaceB`.
- [`wiring/duplicate-keys/for_loop_bare_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/for_loop_bare_key.rs) — a bare-key `for` loop beside a namespace join; the `.rust.stderr` pins the same generic `E0119` pair with the conflicting caret on the loop's `Key: Value` line, the `.cgp.stderr` the `[CGP-E006]` headline naming `DefaultNamespace` and the loop's `GreeterTable`.

## Related

- [Namespace override conflict](namespace-override-conflict.md) — the sibling namespace `E0119`, a *specific*-versus-blanket overlap where one side names a concrete key (a path or a component marker); contrast it by the generic-versus-generic types this class prints.
- [Conflicting wiring](conflicting-wiring.md) — the general `E0119`/`E0428` class for a key or name declared twice, and the full account of Rust's coherence reasoning ([RFC 2451](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html)) that this class specializes.
- [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md), [Namespace inheritance cycle](namespace-inheritance-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md), [`DefaultNamespace`](../../reference/traits/default_namespace.md), and the [namespaces guide](../../guides/namespaces-and-prefixes.md) — the `namespace`/`for` statements whose blanket impls overlap.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0119` entry in the decoder.
