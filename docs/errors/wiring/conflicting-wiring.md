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

The same shape appears with an `open` header colliding with an explicit mapping, a `@`-path duplicated under a `namespace` header, a duplicate `cgp_namespace!` entry, a `#[cgp_component]`-derived marker clashing with a hand-declared type, a duplicate `#[default_impl]` key, or a duplicate `check_components!` entry.

Two namespace collisions that also produce `E0119` are documented separately, because their mistake is not a duplicate declaration but a namespace usage: two blanket forwardings that overlap (joining two namespaces, or a namespace join plus a bare-key `for` loop) are the [overlapping namespace forwarding](namespace-forwarding-conflict.md) class, and a specific entry that overrides a key a namespace already claims (a context re-wiring a registered path, or a child namespace redefining an inherited entry) is the [namespace override conflict](namespace-override-conflict.md) class. This document covers the plain duplicate-or-overlapping *declaration*.

## The raw diagnostic

This section describes what plain `cargo check` prints — the fallback when `cargo-cgp` is not on hand; [How cargo-cgp presents it](#how-cargo-cgp-presents-it) below covers the readable form. There are two error codes, by whether the collision is between *impls* or between *definitions*. A duplicate key or overlapping generic produces **[`E0119`](../error_codes/e0119.md) conflicting implementations**; a duplicate generated *name* produces **[`E0428`](../error_codes/e0428.md) "the name … is defined multiple times"**. Both point precisely: `E0119` carries two carets — "first implementation here" on the earlier entry and "conflicting implementation for `<Type>`" on the later one, aimed at the offending keys rather than the whole block — and `E0428` carries "previous definition here" / "redefined here".

How many `E0119`s one duplicate produces depends on how many impls the entry generates, and knowing this lets a reader treat the pair as one logical conflict rather than two mistakes. A **context-wiring** entry — a `delegate_components!` mapping, or a `namespace`/`open`/`for` header — generates *both* a `DelegateComponent` table impl and an [`IsProviderFor`](../../reference/traits/is_provider_for.md) forwarding impl (so dependency errors stay diagnosable through checks), so a duplicate yields a **pair** of `E0119`s at the same caret, one for each trait. An entry that generates only a single lookup impl yields a **single** `E0119`: a duplicate [`#[default_impl]`](../../reference/traits/default_namespace.md) conflicts on its one `DefaultImpls…` impl, and a duplicate [`cgp_namespace!`](../../reference/macros/cgp_namespace.md) body entry (a `:` mapping or a `=>` redirect) conflicts on its one namespace-trait impl. A duplicated provider *name* is the compound case: `E0428` on the struct, plus the `E0119` pair on the provider's own provider-trait and `IsProviderFor` impls.

Two wrinkles round out the shape, and the second is where Rust's coherence reasoning shows through. When the key is a `@`-path, the conflicting trait's name expands into a long `PathCons<Symbol<…>>` type that dominates the message — the caret still lands on the path leaf, so read the caret, not the type. And a collision in which one impl is a *blanket* — such as an `open` header, which lowers to `impl<Key> …<Key> for Ctx` over every key, colliding with a specific mapping — often carries an extra "downstream crates may implement …" note. That note is `rustc` making its coherence rule explicit, not a second, separate problem. Coherence forbids two impls that *could* both apply to some type, and it reasons about types a downstream crate might add, not only the types in scope; because the blanket's applicability to a given key hinges on a trait bound a downstream crate could later satisfy, the compiler cannot prove the two impls will never overlap, so it rejects them and cites the hypothetical future impl as the reason. This future-compatibility (negative-reasoning) rule is what [RFC 2451](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html) formalizes. The two namespace classes split off above turn on the same reasoning: the [override conflict](namespace-override-conflict.md) carries this note where a specific entry meets a namespace's blanket forwarding, while the [forwarding conflict](namespace-forwarding-conflict.md) is a pure blanket-versus-blanket overlap that prints fully-generic `DelegateComponent<_>` / `IsProviderFor<_, _, _>` carets with no downstream note.

## Where the root cause is

The root cause is **present and precise**: the two carets name the two conflicting entries directly, and the error code names the kind of collision. This is a *structural* class, not a hidden or cascading one, so there is no note chain to walk and no suppressed cause to recover — the diagnostic points at exactly the two lines to reconcile. The only reading skill it demands is ignoring the expanded `PathCons<…>` type on a path-key conflict and trusting the caret.

## How cargo-cgp presents it

`cargo-cgp` keeps the `E0119` code but rewrites the headline to name the *kind* of collision and de-duplicates the pair into one message. Because this is a structural class, there is no `root cause:` tree: both `rustc` carets are preserved (they are already the answer), the redundant `IsProviderFor` half of a generated pair is suppressed so one duplicate reads as one conflict, and any `@`-path key is resugared to bare `@a.b.*` notation instead of the `PathCons<Symbol<…>>` spine. Which `[CGP-E00x]` code it stamps distinguishes the four shapes this class produces:

- A duplicate key or overlapping generic becomes **`[CGP-E004]` duplicate wiring** — `` [CGP-E004] duplicate wiring for component `GreeterComponent` on `Person` `` (or, for a `@`-path, `` duplicate wiring for `@cgp.core.error.ErrorTypeProviderComponent.*` on `App` ``).
- An `open` header colliding with an explicit mapping becomes **`[CGP-E007]` redirect collision** — `` [CGP-E007] component `GreeterComponent` on `Person` is redirected to `@GreeterComponent` `` — carrying a `help:` that names the redirected key to wire the provider under (`` wire the provider `GreetHello` with the key `@GreeterComponent` ``).
- The same key redirected twice — two `=>` mappings on a context, or one `@`-path registered twice inside a `cgp_namespace!` block — becomes **`[CGP-E008]` duplicate redirect**, naming both targets when they differ (`` duplicate redirect for component `FooComponent` on `App`: redirected to both `@app.foo` and `@app.bar` ``).

Two forms in this class are **pass-throughs** that `cargo-cgp` does not rewrite, so a reader sees the raw diagnostic above. The `E0428` name clashes (`duplicate_component_name`, `duplicate_provider_name`) keep `rustc`'s uncoded `E0428` — for a duplicate provider, the surviving `E0119` on the provider trait is kept too, with its `IsProviderFor` half suppressed — because `E0428` already points precisely at the two definitions. A duplicate `#[default_impl]` key is likewise uncoded: its lone `E0119` on the `DefaultImpls…` impl passes through, a small gap where the `[CGP-E004]` rewrite does not yet reach. The codes are defined in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

Remove one of the two entries. For a duplicate check-trait name from two tables over one context, add a `#[check_trait(Name)]` to one. The namespace collisions split off above have their own remedies — inheriting rather than joining several namespaces, or overriding only a path the namespace leaves unbound — covered in [overlapping namespace forwarding](namespace-forwarding-conflict.md) and [namespace override conflict](namespace-override-conflict.md).

## Notes for tooling

`cargo-cgp` already does the collapsing and resugaring this class needs (above); what remains is the pass-through set — the `E0428` name clashes and the duplicate `#[default_impl]` key, all kept uncoded. Those are already precise enough to relay verbatim, but folding the `default_impl` `E0119` into `[CGP-E004]` would make the family complete.

## Backing fixtures

The `.rust.stderr` snapshot pins the raw `E0119`/`E0428` shape and the `.cgp.stderr` the reshaped headline. The `E0119` conflicts `cargo-cgp` rewrites:

- [`wiring/duplicate-keys/duplicate_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_key.rs) and [`wiring/duplicate-keys/duplicate_key_same_block.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_key_same_block.rs) — the same key mapped twice, across two blocks and within one; the `.rust.stderr` pins the `DelegateComponent` + `IsProviderFor` pair of carets, the `.cgp.stderr` the single `[CGP-E004]` headline.
- [`wiring/duplicate-keys/overlapping_generic.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/overlapping_generic.rs) — a generic `<T> Wrapper<T>` table overlapping a specific `Wrapper<u64>` table, also `[CGP-E004]`.
- [`wiring/duplicate-keys/duplicate_open_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_open_key.rs) — an `open` header colliding with an explicit mapping; the `.rust.stderr` carries the "downstream crates may implement" note, the `.cgp.stderr` the `[CGP-E007]` redirect-collision headline with its `help:` naming the `@GreeterComponent` key.
- [`wiring/duplicate-keys/duplicate_redirect.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_redirect.rs) — the same component redirected to two paths on one context, rewritten to `[CGP-E008]` naming both targets.
- [`wiring/namespace-paths/delegate_duplicate_path_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/delegate_duplicate_path_key.rs) — a duplicated `@`-path key whose raw conflicting trait name is a long `PathCons<Symbol<…>>` type; the `.cgp.stderr` resugars it to the bare `@cgp.core.error.ErrorTypeProviderComponent.*` path under `[CGP-E004]`.
- [`wiring/namespace-paths/namespace_duplicate_path_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/namespace_duplicate_path_key.rs) — the same `@`-path redirected twice inside one `cgp_namespace!` block, a single `E0119` on the namespace trait rewritten to `[CGP-E008]`.

The two namespace collisions that are *not* duplicate declarations live with their own classes: `override_registered_path.rs` and `inherited_override_conflict.rs` under [namespace override conflict](namespace-override-conflict.md), and `for_loop_bare_key.rs` and `two_namespaces_joined.rs` under [overlapping namespace forwarding](namespace-forwarding-conflict.md).

The pass-through conflicts `cargo-cgp` leaves uncoded:

- [`wiring/namespace-paths/duplicate_default_impl.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/duplicate_default_impl.rs) — two `#[default_impl]` registering the same key, a *single* `E0119` on the one `DefaultImpls1` impl; the `.cgp.stderr` matches the `.rust.stderr`, the gap noted above.
- [`wiring/duplicate-keys/duplicate_component_name.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_component_name.rs) — a derived `…Component` marker clashing with a hand-declared type; `E0428` kept verbatim.
- [`wiring/duplicate-keys/duplicate_provider_name.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/duplicate-keys/duplicate_provider_name.rs) — two `#[cgp_impl(new …)]` declaring the same provider struct; the `.cgp.stderr` keeps the `E0428` plus the surviving `E0119` on the provider trait `Greeter<_>` (its `IsProviderFor` half suppressed), all uncoded.

## Related

- [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [Overlapping namespace forwarding](namespace-forwarding-conflict.md) and [Namespace override conflict](namespace-override-conflict.md) — the two namespace `E0119` collisions split off from this class, by usage rather than by error code.
- [`delegate_components!`](../../reference/macros/delegate_components.md), [`cgp_namespace!`](../../reference/macros/cgp_namespace.md), and [`DelegateComponent`](../../reference/traits/delegate_component.md).
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0119`/`E0428` entries in the decoder.
