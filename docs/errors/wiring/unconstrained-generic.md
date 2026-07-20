# Unconstrained generic

A per-entry generic parameter appears only in the provider value and never reaches the key, so the generated impl leaves the parameter unconstrained and the compiler rejects it with `E0207`.

## What triggers it

A `delegate_components!` entry may introduce its own generic parameters, but a per-entry generic is well-formed only when it reaches the *key* — where `DelegateComponent<Key<…>>` binds it. Writing one that appears only in the provider value never binds it, and CGP lowers the entry faithfully rather than second-guessing it, so the compiler rejects the free parameter exactly as it would a hand-written impl with an unused parameter.

```rust
delegate_components! {
    Person {
        <T> GreeterComponent: GreetWith<T>, // T is in the value, never the key
    }
}
// lowers to an impl with an unconstrained parameter:
impl<T> DelegateComponent<GreeterComponent> for Person {
    type Delegate = GreetWith<T>; // T constrains nothing
}
```

The same shape arises when a *generic* provider is registered as a per-type default, since the provider's parameter lands only in the `Delegate` associated-type position.

## The raw diagnostic

This section describes what plain `cargo check` prints — the fallback when `cargo-cgp` is not on hand; [How cargo-cgp presents it](#how-cargo-cgp-presents-it) below covers the readable form. The compiler reports **`E0207`** — "the type parameter `T` is not constrained by the impl trait, self type, or predicates" — with the caret on the `<T>` the user wrote in the entry. The class is well-localized rather than cascading, but it is not quite a single line: the entry lowers into more than one generated position carrying `T`, so the same `E0207` prints twice, each with a different `help:` — one offering to move `T` onto the `Person` type, one to remove the unused parameter. Both carets land on the same `<T>` and name the same cause, so there is no note chain and nothing to trace.

The rule behind it is that an impl parameter must be *determined* by the impl, and knowing why makes the fix obvious. Rust requires every generic parameter on an impl to appear in the implemented trait, in the self type, or in a `where`-clause predicate that pins it as an associated type — one of the three "constrained" positions the message names — because otherwise, given a trait reference, the compiler could not decide which `T` the impl is for. Here `T` reaches only the `Delegate` *value* (`GreetWith<T>`), an associated-type position on the right of the `=`, which does not constrain the impl: `DelegateComponent<GreeterComponent>` names no `T`, so any `T` would satisfy the header equally. Permitting that would also break coherence, since a downstream crate adding another `GreetWith<U>` would make the choice ambiguous. This is the rule introduced by [RFC 447](https://github.com/rust-lang/rfcs/pull/447) ("prohibit unused type parameters in impls"); the [`E0207`](../error_codes/e0207.md) reference summarizes it. It is the same negative-reasoning concern that underlies the [orphan rule](orphan-rule.md) and coherence conflicts — an impl must be resolvable no matter what impls other crates add later.

## Where the root cause is

The root cause is **present and precise**: the caret sits on the offending generic parameter, and the message states exactly why it is rejected. This is the most localized class in the catalog — the diagnostic needs no tracing and hides nothing. The only thing the raw message lacks is the CGP-specific remedy, since it describes the constraint in impl terms rather than in terms of the wiring entry.

## How cargo-cgp presents it

`cargo-cgp` does not yet rewrite this class — it passes rustc's diagnostic through unchanged. For the fixture, the tool's `.cgp.stderr` is byte-for-byte its raw `.rust.stderr`: both `E0207` errors, both `help:` notes, no `[CGP-Exxx]` code stamped. That pass-through is why the fixture sits in cargo-cgp's `usability/` tier rather than `acceptable/`. The cause is already pinpoint-accurate at the caret, so nothing is buried — but the message still speaks in impl terms ("the type parameter `T` is not constrained"), where the actionable reading is a wiring one: the generic must reach the component *key* (`<T> SomeKey<T>: …`), not only the provider *value*. Restating the fix that way is the one improvement left for the tool; there is nothing to recover, only to translate. The codes it stamps on the classes it does rewrite are defined in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

Make the generic reach the key so it is bound — introduce it on a key that carries it (`<T> SomeKey<T>: …`) rather than only on the value — or, when the intent was a single concrete wiring, register a concrete provider with no per-entry generic at all. For a generic provider registered as a per-type default, register a concrete provider instead, since the default position cannot bind the provider's parameter.

## Backing fixtures

- [`usability/wiring/constraints/unconstrained_generic.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/constraints/unconstrained_generic.rs) — a per-entry generic (`<T> GreeterComponent: GreetWith<T>`) that appears only in the value, lowering to an impl with an unconstrained `T`; its `.rust.stderr` pins the `E0207` carets on the `<T>` (the error printed twice), and its `.cgp.stderr` is identical — the pass-through that places the fixture in the `usability/` tier, since cargo-cgp does not yet restate this class in wiring terms.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md) — the sibling structural classes.
- [`delegate_components!`](../../reference/macros/delegate_components.md) and [`DelegateComponent`](../../reference/traits/delegate_component.md).
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0207` entry in the decoder.
