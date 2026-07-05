# Check-trait failure (surfaced)

A check forces an unsatisfied impl-side dependency through `IsProviderFor`, so the compiler reports the real missing bound (`E0277`) at the wiring site — the surfaced counterpart of the [hidden unsatisfied dependency](../hidden/unsatisfied-dependency.md).

> **Status: planned.** This class is scaffolded but not yet fully written. The scope and backing below are recorded so the campaign can fill it in; see [../README.md](../README.md) and [../AGENTS.md](../AGENTS.md).

## Intended scope

This document covers the diagnostic a [`check_components!`](../../reference/macros/check_components.md) or `delegate_and_check_components!` assertion produces when a checked component's provider has a dependency the context cannot meet. The check asserts `CanUseComponent`, which requires `IsProviderFor` as a direct bound, defeating the suppression heuristic that hides the [consumer-trait form](../hidden/unsatisfied-dependency.md). The document should record:

- **The diagnostic** — an `E0277` note chain, `CanUseComponent` at the top, walking through `IsProviderFor` and each intervening trait (`HasName`, …) down to the concrete missing bound (`HasField<Symbol!("name")>`). Surfaced, not hidden.
- **Where the root cause is** — present, near the *end* of the note chain (the innermost failing bound is reported last), and the caret lands on the checked component inside the `check_components!` block rather than on the context.
- **Notes for tooling** — the headline a `cargo-cgp` tool should extract is the last note's bound; the intervening `IsProviderFor`/`CanUseComponent` frames are the noise to suppress.

## Backing fixtures

- [acceptable/check_components/missing_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/missing_dependency.rs) — the surfaced `E0277` chain ending at `HasField<Symbol!("name")>`, contrasted against its hidden `delegate_components` counterpart.

## Related

- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the hidden counterpart; the two are the two halves of one phenomenon.
- [Verbose dependency cascade](verbose-cascade.md) — what a surfaced failure looks like when many providers depend transitively on one cause.
- [Debugging CGP compile errors](../../guides/debugging.md) and the [check-traits concept](../../concepts/check-traits.md).
