# Conflicting wiring

The same component key or generated name is wired or declared twice, so the expansion emits two overlapping impls and fails with a coherence error (`E0119`) or a duplicate-definition error (`E0428`).

> **Status: planned.** This class is scaffolded but not yet fully written. The scope and backing below are recorded so the campaign can fill it in; see [../README.md](../README.md) and [../AGENTS.md](../AGENTS.md).

## Intended scope

This document covers the whole-program structural conflicts CGP defers to the compiler because no single expansion can see the other. The document should record each conflict shape and the caret behavior (each caret aimed at the offending entry, not the whole block):

- **Duplicate `DelegateComponent` key** (`E0119`) — the same component mapped twice, whether by two entries in one block, two separate `delegate_components!` blocks, an `open` header colliding with an explicit mapping, or a `@`-path duplicated under a `namespace` header.
- **Overlapping generic entry** (`E0119`) — a `<T> Wrapper<T>` table and a specific `Wrapper<u64>` table wiring the same component overlap, since stable Rust has no specialization.
- **Overriding a namespace-registered path** (`E0119`) — a context that joins a namespace and also directly wires a path the namespace itself registers.
- **Duplicate namespace key** (`E0119`) — the same key mapped twice in one `cgp_namespace!` block.
- **Duplicate generated name** (`E0428`) — two `#[cgp_impl(new Foo)]` declaring `struct Foo` twice, a `#[cgp_component]`-derived marker clashing with a hand-declared one, or two check tables deriving the same `__Check{Context}` name without a `#[check_trait]` override.
- **Duplicate check entry** (`E0119`) — the same component listed twice in a check table.

For each, record whether the root cause is present (it is — the error code and the twin carets name it directly) and note that this class is *structural*, not hidden or cascading, so tooling mainly needs to map the code back to the two conflicting entries.

## Backing fixtures

The existing fixtures to index (currently cross-linked from the owning macros' implementation documents, to be re-pointed here during migration):

- `acceptable/delegate_components/duplicate_key.rs`, `duplicate_key_same_block.rs`, `duplicate_open_key.rs`, `duplicate_path_key.rs`, `overlapping_generic.rs`
- `acceptable/cgp_impl/duplicate_provider_name.rs`, `duplicate_default_impl.rs`
- `acceptable/cgp_component/duplicate_component_name.rs`
- `acceptable/cgp_namespace/duplicate_path_key.rs`, `override_registered_path.rs`

This directory already holds close to a dozen conflict fixtures across macros; when it grows further, split by conflict shape per [../AGENTS.md](../AGENTS.md).

## Related

- [Orphan-rule violation](orphan-rule.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0119`/`E0428` entries in the decoder.
- [`delegate_components!`](../../reference/macros/delegate_components.md), [`cgp_namespace!`](../../reference/macros/cgp_namespace.md), [`DelegateComponent`](../../reference/traits/delegate_component.md).
