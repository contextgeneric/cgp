# Namespace override conflict

Wiring tries to *override* a key or path that a namespace already claims — a context re-wiring a path its joined namespace registers, or a child namespace redefining an entry it inherits — and the specific entry collides with the namespace's blanket impl, so the compiler rejects the overlap with `E0119`.

## What triggers it

This class is the failure of a natural-seeming intent: "the namespace sets this, but I want something different here." A namespace supplies entries as *defaults*, and the inheritance-with-override pattern lets a context shadow one — but only for a key the namespace *routes* to without *terminating*. A key the namespace itself binds (through a `:` body entry, a [`#[default_impl]`](../../reference/traits/default_namespace.md), or an inherited entry) is already covered by the namespace's blanket impl, so a second, more specific impl for that same key overlaps it. The overlap takes two shapes.

The **context-level** shape is a context that joins a namespace and then directly wires a path the namespace registers:

```rust
#[cgp_impl(new GreetHello)]
#[default_impl(@app.GreeterComponent in AppNamespace)] // AppNamespace binds this path
impl Greeter { /* … */ }

delegate_components! {
    App {
        namespace AppNamespace;

        @app.GreeterComponent: GreetBye, // tries to override a path the namespace binds
    }
}
```

The **namespace-level** shape is a child namespace that inherits a parent and redefines one of the parent's keys:

```rust
cgp_namespace! {
    new BaseNs {
        GreeterComponent: GreetHello,
    }
}

cgp_namespace! {
    new ChildNs: BaseNs {
        GreeterComponent: GreetBye, // tries to override an inherited entry
    }
}
```

Both reduce to "a specific entry for a key the namespace's blanket impl already covers." In the context case the blanket impl is the `namespace N;` forwarding `impl<Key> DelegateComponent<Key> for App where Key: N<App>`; in the namespace case it is the inheritance forwarding `impl<Table, Key, Value> ChildNs<Table> for Key where Key: BaseNs<…>`. Either way, CGP lowers both the blanket impl and the specific entry faithfully and cannot see from one macro invocation that they claim the same key, so it defers the overlap to the compiler.

## The diagnostic

This is a **structural** class reported as **[`E0119`](../error_codes/e0119.md) conflicting implementations**, with the two-caret shape — "first implementation here" on the namespace's blanket source, and "conflicting implementation" on the specific entry — landing on the entries the user wrote. What the specific side of the overlap *names* is the signature that tells this class from the generic-versus-generic [overlapping namespace forwarding](namespace-forwarding-conflict.md): here one impl is keyed on a *concrete* key. The two shapes differ in the details, and recognizing each on sight tells a reader which override they attempted.

The **context-level** shape produces a **pair** of `E0119`s, because a context is wired through both the `DelegateComponent` table and the `IsProviderFor` forwarding: one conflict on `DelegateComponent<PathCons<…>>` for `App` and one on `IsProviderFor<PathCons<…>, _, _>` for `App`, the conflicting key expanded into the long `PathCons<Symbol<…>>` path type (read the caret, not the type). This shape additionally carries a `note: downstream crates may implement trait IsProviderFor<PathCons<…>, _, _> for type GreetHello` (and for the overriding provider), which is `rustc` making its coherence reasoning explicit: whether the namespace's blanket forwarding and the direct path entry overlap hinges on whether the redirect's delegate provider implements `IsProviderFor` for that path, a bound a downstream crate could add, so the compiler cannot rule the overlap out and cites the hypothetical impl. That future-compatibility (negative-reasoning) rule is the same one [RFC 2451](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html) formalizes and that the [conflicting wiring](conflicting-wiring.md) class explains in full.

The **namespace-level** shape produces a **single** `E0119` — `conflicting implementations of trait ChildNs<_> for type GreeterComponent` — and **no** downstream note. There is only one conflict because a `cgp_namespace!` block emits only its own lookup-trait impls (`ChildNs<Table> for Key`), never the context-side `DelegateComponent`/`IsProviderFor` pair, so nothing doubles it. There is no downstream note because the overlap is provable locally: the inheritance blanket impl covers `GreeterComponent` exactly when `GreeterComponent: BaseNs<…>`, an impl the same crate already emitted for the parent, so the compiler constructs the conflict without reasoning about any future impl. The self type of the conflict is the *component marker* (`GreeterComponent`), not the context, which is the tell that the collision is inside the namespace's own table rather than on a context.

## Where the root cause is

The root cause is **present and precise** in both shapes: the two carets name the namespace source and the overriding entry, and the error code names the collision. This is a structural class with no note chain to walk and nothing suppressed. The one reading skill it asks for is to look past the expanded `PathCons<Symbol<…>>` key type on the context-level shape and trust the caret, and to read the `downstream crates may implement …` note as coherence's *reason*, not a second, separate problem. What neither shape states is the CGP-specific remedy, since the message frames a namespace override decision as a bare coherence conflict.

## Resolving it

The fix depends on which override was attempted, and both follow one rule: **a namespace entry, once bound, cannot be overridden — only a path the namespace leaves unbound is overridable.**

For the **context-level** shape, override by targeting a path the namespace *routes to* but does not itself *terminate*: register the component's [`#[prefix]`](../../reference/macros/cgp_namespace.md) redirect in a base namespace the context inherits, and leave the leaf path unclaimed so the context can supply it directly. If the namespace genuinely binds the path (a `:` body entry or a `#[default_impl]`), it is not overridable on the context — change it in the namespace instead, or move the binding out of the namespace so the leaf stays open. The [namespaces guide](../../guides/namespaces-and-prefixes.md) works this through: `MockApp` overrides `@app.finance.MoneyTransferrerComponent` precisely because `MockNamespace` deliberately does not register that path.

For the **namespace-level** shape, do not bind the key in the base and redefine it in the child. To vary a key per configuration, leave it *unbound* in the shared base namespace and bind it in each inheriting namespace, so each child supplies the key without overriding an inherited one — the separation the guide recommends between a base namespace that describes an application's *structure* and inheriting namespaces that each describe one *configuration*.

## Notes for tooling

For a `cargo-cgp`-style post-processor, the value is translating the coherence conflict into the override intent it actually represents. On the **context-level** shape the tool should **collapse the `PathCons<Symbol<…>>` key back to its readable `@app.…` path**, recognize the `DelegateComponent` + `IsProviderFor` pair as one logical conflict, and report "the namespace already binds `@app.GreeterComponent`; a context cannot override a path the namespace terminates — route through it and terminate on the context instead." On the **namespace-level** shape, recognizing the self type is a *component marker* and the trait is the child namespace lets the tool report "`ChildNs` cannot override `GreeterComponent`, which it inherits from `BaseNs`; leave it unbound in `BaseNs` to vary it per child." The `downstream crates may implement …` note is coherence scaffolding to suppress once its meaning is folded into the headline.

## Backing fixtures

- [acceptable/cgp_namespace/override_registered_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/override_registered_path.rs) — the context-level shape: a context joining `AppNamespace` overrides a path bound with `#[default_impl]`; its `.stderr` pins the `E0119` pair on `DelegateComponent<PathCons<…>>` and `IsProviderFor<PathCons<…>, _, _>` for `App`, with the expanded path type and the `downstream crates may implement` note.
- [acceptable/cgp_namespace/inherited_override_conflict.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/inherited_override_conflict.rs) — the namespace-level shape: a child namespace redefining an inherited key; its `.stderr` pins the single `E0119` on `ChildNs<_>` for `GreeterComponent`, with "first implementation here" on the inherited parent and no downstream note.

## Related

- [Overlapping namespace forwarding](namespace-forwarding-conflict.md) — the sibling namespace `E0119`, a *blanket*-versus-blanket overlap over every key (fully-generic `DelegateComponent<_>`); contrast it by the concrete key one side names here.
- [Conflicting wiring](conflicting-wiring.md) — the general `E0119`/`E0428` class for a key or name declared twice, and the full account of the RFC 2451 coherence reasoning behind the `downstream crates may implement` note.
- [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md), [Namespace inheritance cycle](namespace-inheritance-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md) (and its Known issues), [`DefaultNamespace`](../../reference/traits/default_namespace.md), and the [namespaces guide](../../guides/namespaces-and-prefixes.md) — the inherit-and-override mechanics and the rule that a bound entry is not overridable.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0119` entry in the decoder.
