# Conflicting wiring

The same component key or generated name is wired or declared twice, so the expansion emits two overlapping impls and the compiler rejects them with a coherence error (`E0119`) or a duplicate-definition error (`E0428`).

## What triggers it

CGP lowers each wiring block independently, with no view of any other block or of the surrounding module, so a collision that only a whole-program view could catch is left to the compiler. The mistake takes several forms, all reducing to "one key or name defined twice":

```rust
// Duplicate key — the same component mapped twice (E0119).
delegate_components! { Person { GreeterComponent: GreetHello } }
delegate_components! { Person { GreeterComponent: GreetGoodbye } }

// Overlapping generic — a generic table and a specific one collide (E0119).
delegate_components! { <T> Wrapper<T> { GreeterComponent: GreetHello } }
delegate_components! { Wrapper<u64>  { GreeterComponent: GreetHello } }

// Duplicate generated name — struct declared twice (E0428, plus E0119 on its impls).
#[cgp_impl(new GreetHello)] impl Greeter { /* … */ }
#[cgp_impl(new GreetHello)] impl Greeter { /* … */ }
```

The same shape appears with an `open` header colliding with an explicit mapping, a `@`-path duplicated under a `namespace` header, a context that joins a namespace and *also* directly wires a path the namespace registers, a duplicate `cgp_namespace!` entry, a `#[cgp_component]`-derived marker clashing with a hand-declared type, a duplicate `#[default_impl]` key, or a duplicate `check_components!` entry.

## The diagnostic

Two error codes, by whether the collision is between *impls* or between *definitions*. A duplicate key or overlapping generic produces **`E0119` conflicting implementations**, and CGP emits it as a *pair* — one conflict for `IsProviderFor<…>` and one for `DelegateComponent<…>` (or the namespace lookup trait) — because both impls are generated per entry. Each error carries two carets: "first implementation here" on the earlier entry and "conflicting implementation for `<Type>`" on the later one, aimed at the offending keys rather than the whole block. A duplicate generated *name* produces **`E0428` "the name … is defined multiple times"** with "previous definition here" / "redefined here"; a duplicated provider struct adds the `E0119` pair on its provider impls on top of the `E0428`.

Two wrinkles are worth recognizing. When the key is a `@`-path, the conflicting trait's name expands into a long `PathCons<Symbol<…>>` type that dominates the message — the caret still lands on the path leaf, so read the caret, not the type. And an `open`-header or namespace-path collision often carries an extra "downstream crates may implement …" note, which is the compiler explaining the coherence overlap in orphan-rule terms rather than a second, separate problem.

## Where the root cause is

The root cause is **present and precise**: the two carets name the two conflicting entries directly, and the error code names the kind of collision. This is a *structural* class, not a hidden or cascading one, so there is no note chain to walk and no suppressed cause to recover — the diagnostic points at exactly the two lines to reconcile. The only reading skill it demands is ignoring the expanded `PathCons<…>` type on a path-key conflict and trusting the caret.

## Resolving it

Remove one of the two entries. The one case with a subtler fix is the context that joins a namespace and also wires a path the namespace itself registers: there, keep the override by targeting a path the namespace *routes to* but does not itself terminate, so the context supplies the leaf without overlapping the namespace's own impl (see [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md) and its Known issues). For a duplicate check-trait name from two tables over one context, add a `#[check_trait(Name)]` to one.

## Notes for tooling

For a `cargo-cgp`-style post-processor this class needs little beyond faithful relaying: the two carets are already the answer, so the tool's job is to **present the pair of conflicting entries** and, on a path-key conflict, to **collapse the `PathCons<Symbol<…>>` type back to its readable `@a.b.c` path** so the headline names the duplicated key rather than a screen of type spine. Recognizing the `E0119`-pair (`IsProviderFor` + `DelegateComponent` for the same key) as one logical conflict, not two, also lets the tool report a single "key wired twice" rather than doubling the count.

## Backing fixtures

The `E0119` conflicts:

- [acceptable/delegate_components/duplicate_key.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/duplicate_key.rs) and [duplicate_key_same_block.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/duplicate_key_same_block.rs) — the same key mapped twice, across two blocks and within one; pins the per-entry carets.
- [acceptable/delegate_components/overlapping_generic.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/overlapping_generic.rs) — a generic `<T> Wrapper<T>` table overlapping a specific `Wrapper<u64>` table.
- [acceptable/delegate_components/duplicate_open_key.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/duplicate_open_key.rs) — an `open` header colliding with an explicit mapping, with the "downstream crates may implement" note.
- [acceptable/delegate_components/duplicate_path_key.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/duplicate_path_key.rs) — a duplicated `@`-path key, whose conflicting trait name expands into the long `PathCons<Symbol<…>>` type.
- [acceptable/cgp_namespace/duplicate_path_key.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/duplicate_path_key.rs) and [override_registered_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/override_registered_path.rs) — a duplicate `cgp_namespace!` entry, and a context overriding a path its joined namespace already registers.
- [acceptable/cgp_impl/duplicate_default_impl.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_impl/duplicate_default_impl.rs) — two `#[default_impl]` registering the same key.

The `E0428` name clashes:

- [acceptable/cgp_component/duplicate_component_name.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_component/duplicate_component_name.rs) — a derived `…Component` marker clashing with a hand-declared type.
- [acceptable/cgp_impl/duplicate_provider_name.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_impl/duplicate_provider_name.rs) — two `#[cgp_impl(new …)]` declaring the same provider struct, adding the `E0119` pair on top of the `E0428`.

## Related

- [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [`delegate_components!`](../../reference/macros/delegate_components.md), [`cgp_namespace!`](../../reference/macros/cgp_namespace.md), and [`DelegateComponent`](../../reference/traits/delegate_component.md).
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0119`/`E0428` entries in the decoder.
