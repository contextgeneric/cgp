# Verbose dependency cascade

One mistake deep in a dependency graph surfaces at every provider that transitively needs it, so the compiler prints one failure per affected provider even though there is a single root cause to fix.

> **Status: planned.** This class is scaffolded but not yet fully written. The scope and backing below are recorded so the campaign can fill it in; see [../README.md](../README.md) and [../AGENTS.md](../AGENTS.md).

## Intended scope

This document covers the *volume* problem rather than any single message: when a `FooProvider` needs a `foo` field, a `BarProvider` depends on `Foo`, and a `BazProvider` depends on `Bar`, a missing `foo` field produces three separate failures naming `FooProvider`, `BarProvider`, and `BazProvider`, and the count reflects the graph's depth, not the number of mistakes. The document should record:

- **The diagnostic** — a repeated `E0277`/`IsProviderFor` cascade, one block per transitively dependent provider, each block structurally similar.
- **Where the root cause is** — present but repeated; the single actionable cause is typically found near the *last or second-to-last* block, since the innermost failing bound is reported after the outer ones. Also cover the elided-`...` case, where the discriminating segment of a long type is written to the `long-type-….txt` file the compiler names in a final note.
- **Resolving it** — fix the one root cause and the whole cascade collapses; bisect or add a narrow `check_components!` to isolate it (see the [debugging guide](../../guides/debugging.md)).
- **Notes for tooling** — a `cargo-cgp` tool should deduplicate the cascade to its distinct root cause and lead with the innermost bound rather than the first (outermost) error printed.

## Backing fixtures

This class needs a **new** multi-provider fixture (a three-deep dependency chain over one missing field) to pin the cascade shape and the root-cause position; none exists yet. Add it under `acceptable/` per [../AGENTS.md](../AGENTS.md) when writing this document.

## Related

- [Check-trait failure (surfaced)](check-trait-failure.md) — the single-cause surfaced diagnostic this class multiplies.
- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the same root cause when hidden by the consumer-trait path.
- [Debugging CGP compile errors](../../guides/debugging.md) — why the error count tells you nothing about the number of mistakes.
