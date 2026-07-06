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

## The diagnostic

The compiler reports a single **`E0207`** — "the type parameter `T` is not constrained by the impl trait, self type, or predicates" — with the caret on the `<T>` the user wrote in the entry. It is one clean, well-localized error, with no note chain and no cascade.

The rule behind it is that an impl parameter must be *determined* by the impl, and knowing why makes the fix obvious. Rust requires every generic parameter on an impl to appear in the implemented trait, in the self type, or in a `where`-clause predicate that pins it as an associated type — one of the three "constrained" positions the message names — because otherwise, given a trait reference, the compiler could not decide which `T` the impl is for. Here `T` reaches only the `Delegate` *value* (`GreetWith<T>`), an associated-type position on the right of the `=`, which does not constrain the impl: `DelegateComponent<GreeterComponent>` names no `T`, so any `T` would satisfy the header equally. Permitting that would also break coherence, since a downstream crate adding another `GreetWith<U>` would make the choice ambiguous. This is the rule introduced by [RFC 447](https://github.com/rust-lang/rfcs/pull/447) ("prohibit unused type parameters in impls"); the [`E0207`](../error_codes/e0207.md) reference summarizes it. It is the same negative-reasoning concern that underlies the [orphan rule](orphan-rule.md) and coherence conflicts — an impl must be resolvable no matter what impls other crates add later.

## Where the root cause is

The root cause is **present and precise**: the caret sits on the offending generic parameter, and the message states exactly why it is rejected. This is the most localized class in the catalog — the diagnostic needs no tracing and hides nothing. The only thing the raw message lacks is the CGP-specific remedy, since it describes the constraint in impl terms rather than in terms of the wiring entry.

## Resolving it

Make the generic reach the key so it is bound — introduce it on a key that carries it (`<T> SomeKey<T>: …`) rather than only on the value — or, when the intent was a single concrete wiring, register a concrete provider with no per-entry generic at all. For a generic provider registered as a per-type default, register a concrete provider instead, since the default position cannot bind the provider's parameter.

## Notes for tooling

This class needs the least tool intervention: the `E0207` is already pinpoint-accurate, so a `cargo-cgp`-style post-processor only needs to **restate the fix in wiring terms** — "the generic `T` on this entry must appear in the component key, not only in the provider" — rather than reformat or recover anything. It is worth recognizing precisely so a tool does *not* treat it like the hidden or cascading classes; there is nothing buried here.

## Backing fixtures

- [acceptable/delegate_components/unconstrained_generic.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/unconstrained_generic.rs) — a per-entry generic (`<T> GreeterComponent: GreetWith<T>`) that appears only in the value, lowering to an impl with an unconstrained `T`; its `.stderr` pins the `E0207` caret on the `<T>`.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md) — the sibling structural classes.
- [`delegate_components!`](../../reference/macros/delegate_components.md) and [`DelegateComponent`](../../reference/traits/delegate_component.md).
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0207` entry in the decoder.
