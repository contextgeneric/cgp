# Unconstrained generic

A per-entry generic parameter appears only in the provider value and never reaches the key, so the generated impl leaves the parameter unconstrained and the compiler rejects it with `E0207`.

> **Status: planned.** This class is scaffolded but not yet fully written. The scope and backing below are recorded so the campaign can fill it in; see [../README.md](../README.md) and [../AGENTS.md](../AGENTS.md).

## Intended scope

This document covers the case where a `delegate_components!` entry introduces a generic that its key does not bind — `<T> GreeterComponent: GreetWith<T>` — so the entry lowers to an impl whose `Delegate` associated type mentions `T` while nothing constrains it. The same shape appears when a *generic* provider is registered as a per-type default, since the provider's parameter lands only in the `Delegate` position. The document should record:

- **The diagnostic** — `E0207` "the type parameter `T` is not constrained by the impl trait, self type, or predicates", with the caret on the `<T>` the user wrote.
- **Where the root cause is** — present and precise; the fix is to make the generic reach the key (`<T> SomeKey<T>: …`) or to register a concrete provider.
- **Notes for tooling** — this is a well-localized structural error; a tool mostly needs to explain the CGP-specific remedy (route the generic through the key) rather than reformat the diagnostic.

## Backing fixtures

- [acceptable/delegate_components/unconstrained_generic.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/unconstrained_generic.rs) — a per-entry generic that appears only in the value, lowering to an impl with an unconstrained `T`.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md) — the sibling structural classes.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0207` entry in the decoder.
- [`delegate_components!`](../../reference/macros/delegate_components.md) and [`DelegateComponent`](../../reference/traits/delegate_component.md).
