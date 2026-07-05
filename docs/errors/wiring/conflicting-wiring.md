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

The same shape appears with an `open` header colliding with an explicit mapping, a `@`-path duplicated under a `namespace` header, a context that joins a namespace and *also* directly wires a path the namespace registers, a `for … in` loop that wires a *bare* key alongside a `namespace` join (both generate a blanket `DelegateComponent` impl over every key), a duplicate `cgp_namespace!` entry, a `#[cgp_component]`-derived marker clashing with a hand-declared type, a duplicate `#[default_impl]` key, or a duplicate `check_components!` entry.

## The diagnostic

Two error codes, by whether the collision is between *impls* or between *definitions*. A duplicate key or overlapping generic produces **`E0119` conflicting implementations**; a duplicate generated *name* produces **`E0428` "the name … is defined multiple times"**. Both point precisely: `E0119` carries two carets — "first implementation here" on the earlier entry and "conflicting implementation for `<Type>`" on the later one, aimed at the offending keys rather than the whole block — and `E0428` carries "previous definition here" / "redefined here".

How many `E0119`s one duplicate produces depends on how many impls the entry generates, and knowing this lets a reader treat the pair as one logical conflict rather than two mistakes. A **context-wiring** entry — a `delegate_components!` mapping, or a `namespace`/`open`/`for` header — generates *both* a `DelegateComponent` table impl and an [`IsProviderFor`](../../reference/traits/is_provider_for.md) forwarding impl (so dependency errors stay diagnosable through checks), so a duplicate yields a **pair** of `E0119`s at the same caret, one for each trait. An entry that generates only a single lookup impl yields a **single** `E0119`: a duplicate [`#[default_impl]`](../../reference/traits/default_namespace.md) conflicts on its one `DefaultImpls…` impl, and a duplicate [`cgp_namespace!`](../../reference/macros/cgp_namespace.md) body entry (a `:` mapping or a `=>` redirect) conflicts on its one namespace-trait impl. A duplicated provider *name* is the compound case: `E0428` on the struct, plus the `E0119` pair on the provider's own provider-trait and `IsProviderFor` impls.

Two wrinkles round out the shape, and the second is where Rust's coherence reasoning shows through. When the key is a `@`-path, the conflicting trait's name expands into a long `PathCons<Symbol<…>>` type that dominates the message — the caret still lands on the path leaf, so read the caret, not the type. And a collision in which one impl is a *blanket* — an `open` header, a `namespace` join, or a bare-key `for` loop, each lowering to `impl<Key> …<Key> for Ctx` over every key — often carries an extra "downstream crates may implement …" note. That note is `rustc` making its coherence rule explicit, not a second, separate problem. Coherence forbids two impls that *could* both apply to some type, and it reasons about types a downstream crate might add, not only the types in scope; because the blanket's applicability to a given key hinges on a trait bound a downstream crate could later satisfy, the compiler cannot prove the two impls will never overlap, so it rejects them and cites the hypothetical future impl as the reason. This future-compatibility (negative-reasoning) rule is what [RFC 2451](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html) formalizes. A pure blanket-versus-blanket overlap — two `for`/`namespace` headers each keyed over every key — needs no such reasoning and prints the bare conflict with fully-generic `DelegateComponent<_>` / `IsProviderFor<_, _, _>` carets and no downstream note.

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
- [acceptable/cgp_namespace/duplicate_path_key.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/duplicate_path_key.rs) and [override_registered_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/override_registered_path.rs) — a duplicate `cgp_namespace!` entry (a *single* `E0119` on the namespace-trait impl), and a context overriding a path its joined namespace already registers (the specific-versus-blanket case, with the "downstream crates may implement" note).
- [acceptable/cgp_namespace/for_loop_bare_key.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/for_loop_bare_key.rs) — a bare-key `for … in` loop alongside a `namespace` join, the pure blanket-versus-blanket case; its `.stderr` pins the fully-generic `DelegateComponent<_>` / `IsProviderFor<_, _, _>` carets with no downstream note.
- [acceptable/cgp_impl/duplicate_default_impl.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_impl/duplicate_default_impl.rs) — two `#[default_impl]` registering the same key, a *single* `E0119` on the one `DefaultImpls1` impl (not a pair).

The `E0428` name clashes:

- [acceptable/cgp_component/duplicate_component_name.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_component/duplicate_component_name.rs) — a derived `…Component` marker clashing with a hand-declared type.
- [acceptable/cgp_impl/duplicate_provider_name.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_impl/duplicate_provider_name.rs) — two `#[cgp_impl(new …)]` declaring the same provider struct, adding the `E0119` pair on top of the `E0428`.

## Related

- [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [`delegate_components!`](../../reference/macros/delegate_components.md), [`cgp_namespace!`](../../reference/macros/cgp_namespace.md), and [`DelegateComponent`](../../reference/traits/delegate_component.md).
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0119`/`E0428` entries in the decoder.
